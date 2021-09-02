#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of existence
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::DispatchResult, pallet_prelude::*, traits::{Currency, Randomness, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_128;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded};

    #[derive(Encode, Decode)]
    pub struct Kitty(pub [u8; 16]);
    type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;

    #[pallet::storage]
    #[pallet::getter(fn on_sale)]
    pub type OnSale<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;
    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> =
    StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type KittyIndex: Parameter + AtLeast32BitUnsigned + Default + Copy + Bounded;
        type MoneyForCreateKitty: Get<BalanceOf<Self>>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreate(T::AccountId, T::KittyIndex),
        KittyTransfer(T::AccountId, T::AccountId, T::KittyIndex),
        OnSale(T::AccountId, T::KittyIndex,  Option<BalanceOf<T>>),
        /// A kitty is sold. (from, to, kitty_id, price)
        Sold(T::AccountId, T::AccountId, T::KittyIndex, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        KittiesCountOverflow,
        NotOwner,
        SameParentIndex,
        InvalidKittyIndex,
        NoEnoughBalance,
        NotForSale,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            T::Currency::reserve(&who,  T::MoneyForCreateKitty::get())
                .map_err(|_| Error::<T>::NoEnoughBalance)?;
            let kitty_id = Self::new_ketty_id()?;
            let dna = Self::random_value(&who);
            Self::created(who, kitty_id, Kitty(dna));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer(
            origin: OriginFor<T>,
            new_owner: T::AccountId,
            kitty_id: T::KittyIndex,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Some(who.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );
            Self::transferred(who, new_owner, kitty_id);
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn breed(
            origin: OriginFor<T>,
            kitty_id1: T::KittyIndex,
            kitty_id2: T::KittyIndex,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(kitty_id1 != kitty_id2, Error::<T>::SameParentIndex);
            let kitty1 = Self::kitties(kitty_id1).ok_or(Error::<T>::InvalidKittyIndex)?;
            let kitty2 = Self::kitties(kitty_id2).ok_or(Error::<T>::InvalidKittyIndex)?;
            let dna1 = kitty1.0;
            let dna2 = kitty2.0;
            let kitty_id = Self::new_ketty_id()?;
            let selector = Self::random_value(&who);
            let mut new_dna = selector.clone();

            for i in 0..selector.len() {
                new_dna[i] = (selector[i] & dna1[i]) | (!selector[i] & dna2[i]);
            }

            Self::created(who, kitty_id, Kitty(new_dna));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn sell(
            origin: OriginFor<T>,
            kitty_id: T::KittyIndex,
            price: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                Some(who.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotOwner
            );
            OnSale::<T>::insert(kitty_id, price);

            Self::deposit_event(Event::OnSale(who, kitty_id, price));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let price = OnSale::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;

            let owner = Owner::<T>::get(kitty_id).unwrap();


            T::Currency::reserve(&who, price).map_err(|_| Error::<T>::NoEnoughBalance)?;

            T::Currency::unreserve(&owner, price);

            T::Currency::transfer(
                &who,
                &owner,
                price,
                frame_support::traits::ExistenceRequirement::KeepAlive,
            )?;

            OnSale::<T>::remove(kitty_id);

            Self::transferred(owner.clone(),who.clone(), kitty_id);
            Self::deposit_event(Event::Sold(owner, who,kitty_id,price));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }
        fn new_ketty_id() -> Result<T::KittyIndex, Error<T>> {
            match Self::kitties_count() {
                Some(id) => {
                    ensure!(
                        id != T::KittyIndex::max_value(),
                        Error::<T>::KittiesCountOverflow
                    );
                    Ok(id)
                }
                None => Ok(0u32.into()),
            }
        }

        fn created(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
            Kitties::<T>::insert(kitty_id, Some(kitty));
            Owner::<T>::insert(kitty_id, Some(owner.clone()));
            KittiesCount::<T>::put(kitty_id + 1u32.into());
            Self::deposit_event(Event::KittyCreate(owner, kitty_id));
        }

        fn transferred(old_owner: T::AccountId, new_owner: T::AccountId, kitty_id: T::KittyIndex) {
            Owner::<T>::insert(kitty_id, Some(new_owner.clone()));
            Self::deposit_event(Event::KittyTransfer(old_owner, new_owner, kitty_id));
        }
    }
}
