use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

//同买同卖，转移给自己，不认为有什么错误

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesCount::<Test>::get(), Some(1));
		assert_eq!(Owner::<Test>::get(0), Some(1));
		let expected_event = super::Event::<Test>::KittyCreate(1,0);
		assert_eq!(
			System::events()[1].event,
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn create_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::create(Origin::signed(3)),Error::<Test>::NoEnoughBalance);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::transfer(Origin::signed(1),2,0));
		assert_eq!(Owner::<Test>::get(0), Some(2));
		let expected_event = super::Event::<Test>::KittyTransfer(1,2,0);
		assert_eq!(
			System::events()[2].event,
			mock::Event::KittiesModule(expected_event)
		);
	});
}


#[test]
fn transfer_when_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 3, 0), Error::<Test>::NotOwner);
	});
}


#[test]
fn breed_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		assert_ok!(KittiesModule::breed(Origin::signed(1), 1, 0));
		assert_eq!(KittiesCount::<Test>::get(), Some(3));
		let expected_event = super::Event::<Test>::KittyCreate(1,2);
		assert_eq!(
			System::events()[4].event,
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn breed_when_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 3), Error::<Test>::InvalidKittyIndex);
	});
}

#[test]
fn breed_when_same_parent() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameParentIndex);
	});
}

#[test]
fn sell_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let price: u128 = 1_500;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		assert_eq!(OnSale::<Test>::get(0), Some(price));
		let expected_event = super::Event::<Test>::OnSale(1,0,Some(price));
		assert_eq!(
			System::events()[2].event,
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn sell_when_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::sell(Origin::signed(3), 0, Some(1000)), Error::<Test>::NotOwner);
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let price: u128 = 1_500;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		assert_ok!(KittiesModule::buy(Origin::signed(2), 0));
		assert_eq!(Owner::<Test>::get(0), Some(2));
		let expected_event = super::Event::<Test>::KittyTransfer(1,2,0);
		assert_eq!(
			System::events()[6].event,
			mock::Event::KittiesModule(expected_event)
		);
		let expected_event = super::Event::<Test>::Sold(1,2,0,price);
		assert_eq!(
			System::events()[7].event,
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn buy_when_not_for_sale() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, None));
		assert_noop!(KittiesModule::buy(Origin::signed(2), 0), Error::<Test>::NotForSale);
	});
}

#[test]
fn buy_when_no_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(2_500)));
		assert_noop!(KittiesModule::buy(Origin::signed(3), 0), Error::<Test>::NoEnoughBalance);
	});
}