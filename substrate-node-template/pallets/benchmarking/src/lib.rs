#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get,
};
use frame_system::ensure_signed;

mod benchmark;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}


decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        Something get(fn something): Option<u32>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        SomethingStored(u32, AccountId),
    }
);


decl_error! {
    pub enum Error for Module<T: Trait> {

        NoneValue,
        StorageOverflow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = T::DbWeight::get().writes(1) + 44_660_000]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
    
            let who = ensure_signed(origin)?;
            debug::info!("-----------who is {:?}", who);
            
            Something::put(something);

            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }
    }
}
