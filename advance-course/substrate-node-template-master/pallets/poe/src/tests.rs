use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;


#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()));
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
	})
}

#[test]
fn revoker_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn revoke_claim_failed_when_is_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 3u64));

		assert_eq!(Proofs::<Test>::get(&claim), (3, frame_system::Module::<Test>::block_number()));

	})
}

#[test]
fn transfer_claim_failed_when_is_transfer_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 23),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_no_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		let claim_temp = vec![2, 3];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim_temp.clone(), 23),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn create_claim_failed_when_claim_length_is_too_larger() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2];

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimLengthTooLarge,
		);
	})
}