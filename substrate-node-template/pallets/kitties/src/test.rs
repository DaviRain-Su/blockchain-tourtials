use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


// 质押，reserve 在讲balance 这个pallet的时候。
// 检查event，就是在测试代码中调用了create函数，检测最后是否正常的抛出了event。
// Self::deposit_event(RawEvent::Created(sender, kitty_id)); 这个是抛出的event，需要在测试代码里面检测是否成功抛出

#[test]
fn owner_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(KittiesModule::create(Origin::signed(1)), Ok(()));
    })
}

// #[test]
// fn owner_kitties_append_values_failed_when_overflow() {
//     new_test_ext().execute_with(|| {
//         run_to_block(4294);
//         assert_noop!(
//             KittiesModule::create(Origin::signed(1)),
//             Error::<Test>::KittiesCountOverflow
//         );
//     })
// }

#[test]
fn test_transfer_success() {
    new_test_ext().execute_with(|| {
        assert_eq!(KittiesModule::transfered(Origin::signed(1), 2, 1), Ok(()));
    })
}

#[test]
fn test_breed_success() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(KittiesModule::create(Origin::signed(1)), Ok(()));
        assert_eq!(KittiesModule::create(Origin::signed(2)), Ok(()));
        assert_eq!(KittiesModule::breed(Origin::signed(1), 1, 2), Ok(()));
    })
}
