use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// 测试存证创建成功
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            (1, frame_system::Module::<Test>::block_number())
        )
    })
}

// 创建存证失败，因为已经有一个同名的存证存在
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            // 不会修改链上的状态
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

// 测试吊销存证成功
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

// 吊销存证但是存证不存在
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::CalimNotExist
        );
    })
}

// 吊销存证但是不是交易的发送方
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

// 测试转移存证成功
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(
            Origin::signed(1),
            claim.clone(),
            23u64
        ));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            (23, frame_system::Module::<Test>::block_number())
        );

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试转移存证，但是转移的发起者不是交易的发送方
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

// 测试转移的存证数据不存在
#[test]
fn transfer_claim_failed_when_claim_no_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        let claim_temp = vec![2, 3];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim_temp.clone(), 23),
            Error::<Test>::CalimNotExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_length_is_too_large() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimLengthTooLarge,
        );
    })
}
