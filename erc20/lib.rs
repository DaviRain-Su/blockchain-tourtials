//! 定义的erc20 合约
//! 
#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;


type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;

#[ink::contract]
mod erc20 {

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

    //定义的错误类型
    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InSufficientBalance,
        InSufficientAllowence,
    }

    type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        
        // 定义的构造函数
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {

            // let caller = Self::env().caller(); 获得合约调用者AccountId
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
                return Err(Error::InSufficientBalance);
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
            
            // 先做判断，判断是不是有足够的余额可以从from到to的余额转账
            let allowance = self.allowance_of(from, to);
            // 如果查到余额不足，抛出错误
            if allowance < value {
                return Err(Error::InSufficientAllowence);
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
                return Err(Error::InSufficientBalance);
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
