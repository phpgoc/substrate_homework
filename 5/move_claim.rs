 #[pallet::weight(0)]
pub fn move_claim(origin: OriginFor<T>,claim:Vec<u8>,to:T::AccountId) -> DispatchResultWithPostInfo{
    let sender = ensure_signed(origin)?;
    let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
    ensure!(owner == sender,Error::<T>::NotClaimOwner);

    Proofs::<T>::insert(
        &claim,
        (to.clone(), frame_system::Pallet::<T>::block_number()),
    );
    Self::deposit_event(Event::ClaimMoved(sender,claim,to));
    Ok(().into())
}
