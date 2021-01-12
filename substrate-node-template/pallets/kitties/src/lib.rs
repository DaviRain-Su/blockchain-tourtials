#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Randomness,
    Parameter, StorageMap, StorageValue,
    debug,
};
use frame_system::ensure_signed;
use sp_io::hashing::blake2_128;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, Bounded, MaybeDisplay, MaybeSerialize, Member},
    DispatchError,
};
use sp_std::fmt::Debug;
use sp_std::vec;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

pub mod dna;
pub mod kitty;

use dna::DNA;
use kitty::{Kitty, Parents};

//trait define
pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
    type KittyIndex: Parameter
        + Member
        + MaybeSerialize
        + Debug
        + Default
        + MaybeDisplay
        + AtLeast32BitUnsigned
        + Copy
        + Bounded;
}

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
        pub KittiesCount get(fn kitties_count): T::KittyIndex;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;
        // 通过kitty total 用来存储映射 从账户所有者映射管理的kitties的token个数
        pub KittyTotal get(fn kitty_total) : map hasher(blake2_128_concat) T::AccountId => vec::Vec<T::KittyIndex>;
    }
}

decl_event!(
    pub enum Event<T> where <T as frame_system::Trait>::AccountId, <T as Trait>::KittyIndex, {
        Created(AccountId, KittyIndex),
        Transferred(AccountId, AccountId, KittyIndex),
    }
);

decl_error! {
   pub enum Error for Module<T: Trait> {
        KittiesAlreadyExist,
        KittiesCountOverflow,
        InvalidKittyId,
        RequireDifferentParent,
   }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;

            debug::info!("kitty id = {:?}", kitty_id);

            let dna = Self::random_value(&sender);
            let kitty = Kitty::new().set_self_dna(dna);
            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
            Ok(())
        }

        #[weight = 0]
        fn transfered(origin, to: T::AccountId, kitty_id: T::KittyIndex) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            <KittyOwners<T>>::insert(kitty_id, to.clone());
            Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
            Ok(())
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?; // error 这里要加上? 将Result中的数据取出来
            Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        let kitty_id = Self::kitties_count(); // just get the value not update

        debug::info!("Kitty_id = {:?}", kitty_id);

        if kitty_id == <T as Trait>::KittyIndex::max_value() {
            return Err(Error::<T>::KittiesCountOverflow.into());
        }
        Ok(kitty_id)
    }

    fn random_value(sender: &T::AccountId) -> DNA {

        let payload = (
            T::Randomness::random_seed(),
            &sender,
            <frame_system::Module<T>>::extrinsic_index(),
        );
        DNA::new().set_value(payload.using_encoded(blake2_128))
    }

    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
        debug::info!("Debug info owner = {:?}, kitty_id = {:?}, kitty = {:?}", owner, kitty_id, kitty.clone());
        <Kitties<T>>::insert(kitty_id, kitty);
        <KittiesCount<T>>::put(kitty_id + 1.into());
        <KittyOwners<T>>::insert(kitty_id, owner);
        // 如果存在 将val值增加，即增加账户对应的kitty个数
        if <KittyTotal<T>>::contains_key(&owner) {
            let _ = <KittyTotal<T>>::mutate(owner, |val| val.push(kitty_id));
        } else {
            // 如果不存在 重新插入一个新的
            <KittyTotal<T>>::insert(owner, vec![kitty_id]);
        }
    }

    fn update_kitty(kitty_id: T::KittyIndex, kitty: Kitty) {
        <Kitties<T>>::insert(kitty_id, kitty);
    }

    // logic
    fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
        (selector & dna1) | (!selector & dna2)
    }

    fn do_breed(
        sender: &T::AccountId,
        kitty_id_1: T::KittyIndex,
        kitty_id_2: T::KittyIndex,
    ) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        debug::info!("sender = {:?}, kitty_id1 = {:?}, kitty_id2 = {:?}", sender, kitty_id_1, kitty_id_2);

        let mut kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let mut kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty1.get_self_dna();
        let kitty2_dna = kitty2.get_self_dna();

        // set partner dna
        kitty1.mutate_partner_dna(kitty2_dna);
        kitty2.mutate_partner_dna(kitty1_dna);

        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            // error not add Self::
            new_dna[i] = Self::combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        // set self dna
        let new_dna = DNA::new().set_value(new_dna);

        // set children
        kitty1.mutate_children_dna(new_dna);
        kitty2.mutate_children_dna(new_dna);

        //update kitty
        Self::update_kitty(kitty_id_1, kitty1);
        Self::update_kitty(kitty_id_2, kitty2);

        // set parents dna
        let parent_dna = Parents::new().set_father(kitty1_dna).set_mother(kitty2_dna);

        // 组装 结构体
        let new_kitty = Kitty::new()
            .set_self_dna(new_dna)
            .set_parents_dna(parent_dna);

        Self::insert_kitty(sender, kitty_id, new_kitty);
        Ok(kitty_id)
    }
}
