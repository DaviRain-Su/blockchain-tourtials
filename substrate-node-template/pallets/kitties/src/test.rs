use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_system::EventRecord;
use frame_system::Phase;

// 质押，reserve 在讲balance 这个pallet的时候。
// 检查event，就是在测试代码中调用了create函数，检测最后是否正常的抛出了event。
// Self::deposit_event(RawEvent::Created(sender, kitty_id)); 这个是抛出的event，需要在测试代码里面检测是否成功抛出

// 测试创建一个Kitty
#[test]
fn owner_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_ok!(KittiesModule::create(Origin::signed(1)));

        assert_eq!(
            System::events(),
            vec![EventRecord {
                phase: Phase::Initialization,
                event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 0)),
                topics: vec![],
            }]
        )
    })
}

// 测试转让Kitty成功
#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_ok!(KittiesModule::transfered(Origin::signed(1), 2, 0));

        assert_eq!(
            System::events(),
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 0)),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Transferred(1u64, 2, 0)),
                    topics: vec![],
                }
            ]
        )
    })
}

// 测试转让Kitty 失败，因为Kitty 不存在
#[test]
fn transfer_kitty_failed_when_no_exists() {
    new_test_ext().execute_with(|| {
        // run_to_block(10);
        // let _ = KittiesModule::create(Origin::signed(1));

        // assert_ok!(KittiesModule::transfered(Origin::signed(1), 2, 0));
        assert_noop!(
            KittiesModule::transfered(Origin::signed(1), 2, 0),
            Error::<Test>::KittyNotExists
        );
    })
}

#[test]
fn transfer_kitty_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::transfered(Origin::signed(2), 3, 0),
            Error::<Test>::NotKittyOwner
        );
    })
}

#[test]
fn transfer_kitty_failed_when_transfer_self() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::transfered(Origin::signed(1), 1, 0),
            Error::<Test>::TransferToSelf
        );
    })
}

#[test]
fn breed_kitty_work() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));
        let _ = KittiesModule::create(Origin::signed(1));

        assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));

        assert_eq!(
            System::events(),
            vec![
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 0)),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 1)),
                    topics: vec![],
                },
                EventRecord {
                    phase: Phase::Initialization,
                    event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 2)),
                    topics: vec![],
                }
            ]
        )
    })
}

#[test]
fn breed_kitty_fail_when_same() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 0),
            Error::<Test>::RequireDifferentParent
        );
    })
}

#[test]
fn breed_kitty_fail_when_not_exists() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 1),
            Error::<Test>::KittyNotExists
        );
    })
}

#[test]
fn breed_kitty_work_when_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let _ = KittiesModule::create(Origin::signed(1));
        let _ = KittiesModule::create(Origin::signed(1));

        assert_noop!(
            KittiesModule::breed(Origin::signed(2), 0, 1),
            Error::<Test>::NotKittyOwner
        );
    })
}
