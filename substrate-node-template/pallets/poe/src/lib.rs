#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// 配置存证vec的长度上限
    type ClaimLength: Get<usize>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------
    trait Store for Module<T: Trait> as TemplateModule {
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        CalimNotExist,
        NotClaimOwner,
        ClaimLengthTooLarge
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        // origin 交易的发送方
        // claim 交易文件的哈希值
        #[weight = 0]
        pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            // 校验交易的发送方是签名的，获取交易发送方的accountId sender
            let sender = ensure_signed(origin)?;

            // 检测交易存证的长度过大
            // 保证插入的存证的数据长度小于或者等于ClaimLength
            ensure!(claim.len() <= T::ClaimLength::get() , Error::<T>::ClaimLengthTooLarge);

            // 校验不存在
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number())); //区块数

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

            Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            // 检测交易存证的长度过大
            ensure!(claim.len() <= T::ClaimLength::get() , Error::<T>::ClaimLengthTooLarge);

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::CalimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        #[weight = 0]
        pub fn transfer_claim(origin, claim: Vec<u8>, dest: T::AccountId) ->dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            // 检测交易存证的长度过大
            ensure!(claim.len() <= T::ClaimLength::get() , Error::<T>::ClaimLengthTooLarge);

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::CalimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::insert(&claim, (dest, frame_system::Module::<T>::block_number()));

            Ok(())
        }
    }
}
