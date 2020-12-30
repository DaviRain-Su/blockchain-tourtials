#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, decl_storage, dispatch};
use frame_system::{self as system, ensure_signed};


pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        Thing1 get(fn thing1): u32;
        Thing2 get(fn thing2): u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn set_thing_1(origin, val: u32) -> dispatch::DispatchResult {
            let _ = ensure_signed(origin)?;

            Thing1::put(val);

            Self::deposit_event(Event::ValueSet(1, val));
            Ok(())
        }

        #[weight = 10_000]
        pub fn set_thing_2(origin, val: u32) -> dispatch::DispatchResult {
            let _ = ensure_signed(origin)?;

            Thing2::put(val);

            Self::deposit_event(Event::ValueSet(2, val));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn get_sum() -> u32 {
        Thing1::get() + Thing2::get()
    }
}

decl_event!(
    pub enum Event {
        ValueSet(u32, u32),
    }
);