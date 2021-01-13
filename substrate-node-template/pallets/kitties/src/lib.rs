#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Randomness,
    traits::Get,
    Parameter, StorageMap, StorageValue,
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
// pub mod kitty;

use dna::DNA;
use frame_support::traits::Currency;
use frame_support::traits::ReservableCurrency;
// use kitty::{Kitty, Parents};

// 定义一个Kitty 数据结构
#[derive(Encode, Decode, Clone, Copy, Debug)]
pub struct Kitty(pub DNA);

impl Kitty {
    pub fn new() -> Self {
        Self { 0: DNA::new() }
    }
    pub fn set_value(self, dna: DNA) -> Self {
        Self { 0: dna, ..self }
    }
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

//trait define
pub trait Trait: frame_system::Trait {
    // 如果有触发事件，就必须包含这一行
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;

    // 定义KittyIndex类型，要求实现指定trait
    // Parameter 表示可用于函数参数传递
    // AtLeast32Bit 表示转换为u32不会造成数据丢失
    // Bounded表示包含上界和下界
    // Default 表示有默认值
    // Copy 表示可以实现Copy 方法
    type KittyIndex: Parameter
        + Member
        + MaybeSerialize
        + Debug
        + Default
        + MaybeDisplay
        + AtLeast32BitUnsigned
        + Copy
        + Bounded;
    // 创建kitty 的时候，需要质押的代币
    type NewKittyReserve: Get<BalanceOf<Self>>;
    // Currency 类型，用于质押等资产相关的操作
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

// 定义数据存储
decl_storage! {
    trait Store for Module<T: Trait> as Kitties {

        // 通过map，记录kittyIndex 与DNA存放内容的一个映射关系， 保存所有的kitty 的数据，用kittyIndex用作键值
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;

        // 记录kitties的数目
        pub KittiesCount get(fn kitties_count): T::KittyIndex;

        // 记录kittyIndex 与所有者AccountId之间的映射， 保存每一只猫的拥有者
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;

        // 通过kitty total 用来存储映射 从账户所有者映射管理的kitties的token个数， 记录拥有者和猫之间的关系
        pub KittyTotal get(fn kitty_total) : map hasher(blake2_128_concat) T::AccountId => vec::Vec<T::KittyIndex>;

        // 通过一个map，记录他的父母的KittyIndex
        // 在breed是更新
        pub KittiesParents get(fn kitty_parents) : map hasher(blake2_128_concat) T::KittyIndex => (T::KittyIndex, T::KittyIndex);

        // 通过一个double map 可以从父母的任何一方的index映射到孩子的index. 使用Vec<KittyIndex>是因为同一对父母可能产生多个孩子
        // 在breed是更新
        pub KittiesChildren get(fn kitty_children): double_map hasher(blake2_128_concat) T::KittyIndex,  hasher(blake2_128_concat) T::KittyIndex => vec::Vec<T::KittyIndex>;

        // 可以通过 Kitties parents和kitties children 得到一个kittyIndex 到 bother(Vec<KittyIndex>)的一个映射关系
        // 在breed是更新
        pub KittiesBrother get(fn kitty_brother): map hasher(blake2_128_concat) T::KittyIndex => vec::Vec<T::KittyIndex>;

        // 通过一个map记录与另一半breed的KittyIndex
        // 在breed是更新
        pub KittiesPartner get(fn kitty_partner) : map hasher(blake2_128_concat) T::KittyIndex => T::KittyIndex;
    }
}

// 定义事件
decl_event!(
    // where 后边的部分，是表示在Event里面需要用的一些类型来自哪个Trait定义
    pub enum Event<T> where <T as frame_system::Trait>::AccountId, <T as Trait>::KittyIndex, {
        Created(AccountId, KittyIndex),
        Transferred(AccountId, AccountId, KittyIndex),
    }
);

// 定义错误类型
decl_error! {
   pub enum Error for Module<T: Trait> {

        KittiesCountOverflow,
        InvalidKittyId,
        RequireDifferentParent,
        KittyNotExists,
        NotKittyOwner,
        TransferToSelf,
        MoneyNotEnough,
   }
}

// 定义可被调用的方法
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // 如果有触发错误信息，必须包含这一行
        type Error = Error<T>;
        // 如果有触发事件，必须包含这一行
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);

            let kitty = Kitty::new().set_value(dna);

            T::Currency::reserve(&sender, T::NewKittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough)?;

            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
            Ok(())
        }

        #[weight = 0]
        fn transfered(origin, to: T::AccountId, kitty_id: T::KittyIndex) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            // 判断KittyIndex是否存在， 通过ok_or将错误抛出， 如果没有返回一个Option类型的数据
            let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::KittyNotExists)?;

            // 判断KittyIndex 是否属于发送者
            ensure!(owner == sender, Error::<T>::NotKittyOwner);
            // 不能转让给自己
            ensure!(to != sender, Error::<T>::TransferToSelf);

            KittyOwners::<T>::insert(kitty_id, &to);

            KittyTotal::<T>::mutate(&sender, |val| val.retain(|&temp| temp == kitty_id));
            KittyTotal::<T>::mutate(&to, |val| val.push(kitty_id));

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
            T::Randomness::random_seed(), // 通过最近区块信息生成的随机数种子
            &sender,
            <frame_system::Module<T>>::extrinsic_index(), // 当前交易在区块中的顺序
        );
        DNA::new().set_value(payload.using_encoded(blake2_128))
    }

    // 插入一个kitty
    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
        // update kitties
        <Kitties<T>>::insert(kitty_id, kitty);

        // update kitties count
        <KittiesCount<T>>::put(kitty_id + 1.into());

        // update kitty owners
        <KittyOwners<T>>::insert(kitty_id, owner);

        // update kitty total
        // 如果存在 将val值增加，即增加账户对应的kitty个数
        if <KittyTotal<T>>::contains_key(&owner) {
            let _ = <KittyTotal<T>>::mutate(owner, |val| val.push(kitty_id));
        } else {
            // 如果不存在 重新插入一个新的
            <KittyTotal<T>>::insert(owner, vec![kitty_id]);
        }
    }

    fn update_kitties_parents(
        children: T::KittyIndex,
        father: T::KittyIndex,
        mother: T::KittyIndex,
    ) {
        <KittiesParents<T>>::insert(children, (father, mother));
    }

    fn update_kitties_children(
        children: T::KittyIndex,
        father: T::KittyIndex,
        mother: T::KittyIndex,
    ) {
        if <KittiesChildren<T>>::contains_key(father, mother) {
            let _ = <KittiesChildren<T>>::mutate(father, mother, |val| val.push(children));
        } else {
            // 如果不存在 重新插入一个新的
            <KittiesChildren<T>>::insert(father, mother, vec![children]);
        }
    }

    fn update_kitties_brother(kitty_id: T::KittyIndex) {
        let (father, mother) = <KittiesParents<T>>::get(kitty_id);

        if <KittiesChildren<T>>::contains_key(father, mother) {
            let val: vec::Vec<T::KittyIndex> = <KittiesChildren<T>>::get(father, mother);
            let reserve_val: vec::Vec<T::KittyIndex> =
                val.into_iter().filter(|&val| val != kitty_id).collect();
            <KittiesBrother<T>>::insert(kitty_id, reserve_val);
        } else {
            <KittiesBrother<T>>::insert(kitty_id, vec::Vec::<T::KittyIndex>::new());
        }
    }

    fn update_kitties_partner(partner1: T::KittyIndex, partner2: T::KittyIndex) {
        <KittiesPartner<T>>::insert(partner1, partner2);
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
        // 保证两个kitty是不一样的
        ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

        // 判断kittyIndex是否存在，通过ok_or将错误跑出来，如果没有将返回一个Option类型的数据
        let owner1 = Self::kitty_owner(kitty_id_1).ok_or(Error::<T>::KittyNotExists)?;
        let owner2 = Self::kitty_owner(kitty_id_2).ok_or(Error::<T>::KittyNotExists)?;

        // 判断KittyIndex是否属于发送者
        ensure!(owner1 == *sender, Error::<T>::NotKittyOwner);
        ensure!(owner2 == *sender, Error::<T>::NotKittyOwner);

        // 根据kitty id 得到 kitty, 通过 pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
        let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        // 获得一个新的kitty id
        let kitty_id = Self::next_kitty_id()?;

        // 更新 partner
        Self::update_kitties_partner(kitty_id_1, kitty_id_2);
        Self::update_kitties_partner(kitty_id_1, kitty_id_2);

        // 更新孩子index对应的父母index
        Self::update_kitties_parents(kitty_id, kitty_id_1, kitty_id_2);

        // 更新double map 父母对应的孩子index
        Self::update_kitties_children(kitty_id, kitty_id_1, kitty_id_2);

        // 更新brother, 依赖于 kitties parents, kitties children
        Self::update_kitties_brother(kitty_id);

        // 拿到两个kitty的内部DNA序列
        let kitty1_dna: DNA = kitty1.0;
        let kitty2_dna: DNA = kitty2.0;

        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            // error not add Self::
            new_dna[i] = Self::combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        // set self dna
        let new_dna = DNA::new().set_value(new_dna);

        let new_kitty = Kitty::new().set_value(new_dna);

        T::Currency::reserve(&sender, T::NewKittyReserve::get()).map_err(|_| Error::<T>::MoneyNotEnough)?;

        Self::insert_kitty(sender, kitty_id, new_kitty);
        Ok(kitty_id)
    }
}
