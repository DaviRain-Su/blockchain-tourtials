use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
use frame_system::Origin;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()))
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!( // 不会修改链上的状态
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}