//! 定义的erc20 合约
//! 
#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{Environment};
use ink_lang as ink;


// type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;

#[ink::chain_extension]
pub trait FetchErc20 {
    type ErrorCode = Erc20Error;

    // #[ink(extension = 1101, returns_result = false)]
    // fn transfer(to: AccountId, value: Balance);
    //
    // #[ink(extension = 1102, returns_result = false)]
    // fn transfer_from(from: AccountId, to: AccountId, value: Balance);
    //
    // #[ink(extension = 1103, returns_result = false)]
    // fn transfer_help(from: AccountId, to: AccountId, value: Balance);
    //
    // #[ink(extension = 1104, returns_result = false)]
    // fn allowance(to: AccountId, value: Balance);

    #[ink(extension = 1101, returns_result = false)]
    fn fetch_random() -> [u8; 32];
}



//定义的错误类型
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Erc20Error {
    InSufficientBalance,
    InSufficientAllowence,
}

impl ink_env::chain_extension::FromStatusCode for Erc20Error {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::InSufficientBalance),
            2 => Err(Self::InSufficientAllowence),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = FetchErc20;
}


#[ink::contract(env = crate::CustomEnvironment)]
mod erc20 {

    use super::Erc20Error;

    use ink_storage::collections::HashMap as StorageHashMap;

    // 定义的存储类型
    #[ink(storage)]
    pub struct Erc20 {
        // 总的供给量
        total_supply: Balance,
        // 用户对应的供给量
        balances: StorageHashMap<AccountId, Balance>,
        // 允许一个账户向一个账户转账的金额
        allowance: StorageHashMap<(AccountId, AccountId), Balance>,
        // randoms
        value: [u8; 32],
    }

    // 定义的事件类型
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }
    #[ink(event)]
    pub struct Approve {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    // for random
    #[ink(event)]
    pub struct RandomUpdated {
        #[ink(topic)]
        new: [u8; 32],
    }

    type Result<T> = core::result::Result<T, Erc20Error>;

    impl Erc20 {
        
        // 定义的构造函数
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {

            // 获得合约调用者AccountId
            let caller = Self::env().caller();
            
            // 在创建时讲总供给量默认与合约的创建者做绑定
            let mut balances = StorageHashMap::<AccountId, Balance>::new();
            balances.insert(caller, total_supply);

            // 合约创建时的调用的事件
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowance: StorageHashMap::new(),
                value: Default::default(),
            }
        }

        // 定义的获得总供给量的函数
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        // 定义的获得某个账户有多少余额的函数
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            *self.balances.get(&account).unwrap_or(&0)
        }

        // 定义的获得owner可以向spend转账的余额
        #[ink(message)]
        pub fn allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        // 定义的合约可以向账户转账的函数
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();

            self.transfer_helper(caller, to, value)?;

            Ok(())
        }

        // 定义的 更新allowance的函数
        #[ink(message)]
        pub fn allowance(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();

            let caller_balance = self.balance_of(caller.clone());

            // 确保caller的持有的value数量一定大于spender持有的value数量的
            if caller_balance < value {
                return Err(Erc20Error::InSufficientBalance);
            }

            // 插入 ：拥有者可以向花费者转移的value
            self.allowance.insert((caller, spender), value);

            // 发送事件
            self.env().emit_event(Approve {
                owner: caller,
                spender: spender,
                value: value,
            });
            Ok(())
        }


        // 定义的一个账户可以向另一个转账的函数
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {

            //先做判断，判断是不是有足够的余额可以从from到to的余额转账
            let allowance = self.allowance_of(from, to);
            // 如果查到余额不足，抛出错误
            if allowance < value {
                return Err(Erc20Error::InSufficientAllowence);
            }

            // 更新allowance函数
            self.allowance.insert((from, to), allowance - value);

            self.transfer_helper(from, to, value)?;

            Ok(())
        }

        // 私有的转账帮助函数
        fn transfer_helper(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {

            // 判断from是不是有足够的余额可以向to转账
            let from_balance = self.balance_of(from.clone());
            if from_balance < value {
                return Err(Erc20Error::InSufficientBalance);
            }

            // 更新from账户对应的balance余额
            self.balances.insert(from, from_balance - value);

            let to_balance = self.balance_of(to);
            // 更新to账户对应的balance余额
            self.balances.insert(to, to_balance + value);

            // 发送事件
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value: value,
            });


            Ok(())
        }

        #[ink(message)]
        pub fn update_random(&mut self) -> Result<()> {
            // Get the on-chain random seed
            let new_random = self.env().extension().fetch_random()?;
            self.value = new_random;
            // emit the RandomUpdated event when the random seed
            // is successfully fetched.
            self.env().emit_event(RandomUpdated { new: new_random });
            Ok(())
        }

        #[ink(message)]
        pub fn get_random(&self) -> [u8; 32] {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn create_works() {
            let erc20 = Erc20::new(999);
            assert_eq!(erc20.total_supply(), 999);
        }

        #[ink::test]
        fn balances_works() {
            let erc20 = Erc20::new(999);
            assert_eq!(erc20.total_supply(), 999);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 999);
        }

        #[ink::test]
        fn transfer_works() {
            let mut erc20 = Erc20::new(999);
            assert_eq!(erc20.total_supply(), 999);
            assert_eq!(erc20.transfer(AccountId::from([0x0; 32]), 100), Ok(()));
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 899);
            assert_eq!(erc20.balance_of(AccountId::from([0x0; 32])), 100);
        }

        #[ink::test]
        fn allowence_works() {
            let mut erc20 = Erc20::new(999);
            assert_eq!(erc20.total_supply(), 999);
            assert_eq!(erc20.allowance(AccountId::from([0x0; 32]), 100), Ok(()));
            assert_eq!(
                erc20.allowance_of(AccountId::from([0x1; 32]), AccountId::from([0x0; 32])),
                100
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut erc20 = Erc20::new(999);
            assert_eq!(erc20.total_supply(), 999);
            assert_eq!(erc20.allowance(AccountId::from([0x1; 32]), 200), Ok(()));
            assert_eq!(
                erc20.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 100),
                Ok(())
            );
        }
    }
}
