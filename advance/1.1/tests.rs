use super::pallet::*;
use crate::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_success() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn create_claim_proof_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn revoke_claim_success() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), None);
    })
}

#[test]
fn revoke_claim_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim),
            Error::<Test>::ClaimNotExist
        );
    });
}

#[test]
fn revoke_claim_not_claim_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim),
            Error::<Test>::NotClaimOwner
        );
    });
}

#[test]
fn transfer_claim_success() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::transfer_claim(
            Origin::signed(1),
            claim.clone(),
            2
        ));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn transfer_claim_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim, 2),
            Error::<Test>::ClaimNotExist
        );
    });
}

#[test]
fn transfer_claim_not_claim_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2, 3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim, 2),
            Error::<Test>::NotClaimOwner
        );
    });
}
