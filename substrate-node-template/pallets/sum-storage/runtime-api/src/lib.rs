#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]


sp_api::decl_runtime_apis! {
    pub trait SumStorageApi {
        fn get_sum() -> u32;
    }
}