#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    
    use ink_storage::collections::HashMap as StorageHashMap;
    

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// Stores a single `bool` value on the storage.
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowance: StorageHashMap<(AccountId,AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value:Balance,
    }
    #[ink(event)]
    pub struct Approve{
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value:Balance,
    }


    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error{
        InSufficientBalance,
        InSufficientAllowence,
    }

    type Result<T>=core::result::Result<T,Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::<AccountId, Balance>::new();
            balances.insert(caller, total_supply);

            Self::env().emit_event(Transfer{
                from: None,
                to: Some(caller),
                value:total_supply,
            });

            Self { 
                total_supply:total_supply,
                balances: balances,
                allowance:StorageHashMap::new(),
            }
        }

        

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn totalsupply(& self) ->Balance{
            self.total_supply
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn balanceof(&self, account: AccountId) -> Balance {
            *self.balances.get(&account).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowanceof(& self, owner:AccountId, spender: AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to:AccountId,value:Balance)-> Result<()>{
            let caller = self.env().caller();
            self.transfer_helper(caller, to,value)?;
            Ok(())
            
        }

        #[ink(message)]
        pub fn allowance(&mut self, spender:AccountId, value:Balance) -> Result<()> {
            let caller = self.env().caller();
            let caller_balance = *self.balances.get(&caller).unwrap_or(&0);
            if caller_balance < value {
                return Err(Error::InSufficientBalance);
            }
            self.allowance.insert((caller, spender), value);
            self.env().emit_event(Approve{
                owner: caller,
                spender:spender,
                value:value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, owner:AccountId, to:AccountId, value:Balance)->Result<()> {
            let caller = self.env().caller();
            let allowance = *self.allowance.get(&(owner,caller)).unwrap_or(&0);
            if allowance < value {
                return Err(Error::InSufficientAllowence);
            }
            self.allowance.insert((owner,caller), allowance- value);
            self.transfer_helper(owner,to,value)?;
            Ok(())
        }

        fn transfer_helper(&mut self, from:AccountId, to:AccountId, value:Balance)-> Result<()> {
            let from_balance = *self.balances.get(&from).unwrap_or(&0);
            if from_balance < value{
                return Err(Error::InSufficientBalance);
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = *self.balances.get(&to).unwrap_or(&0);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer{
                from:Some(from),
                to:Some(to),
                value:value,
            });
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_lang as ink;
        /// We test if the default constructor does its job.
        #[ink::test]
        fn create_works() {
            let erc20 = Erc20::new(999);
            assert_eq!(erc20.totalsupply(), 999);
        }

        #[ink::test]
        fn balances_works() {
            let erc20 = Erc20::new(999);
            assert_eq!(erc20.totalsupply(), 999);
            assert_eq!(erc20.balanceof(AccountId::from([0x1; 32])), 999);
        }

        #[ink::test]
        fn transfer_works() {
            let mut erc20 = Erc20::new(999);
            assert_eq!(erc20.totalsupply(), 999);
            assert_eq!(erc20.transfer(AccountId::from([0x0; 32]),100), Ok(()));
            assert_eq!(erc20.balanceof(AccountId::from([0x1; 32])), 899);
            assert_eq!(erc20.balanceof(AccountId::from([0x0; 32])), 100);
        }

        #[ink::test]
        fn allowence_works() {
            let mut erc20 = Erc20::new(999);
            assert_eq!(erc20.totalsupply(), 999);
            assert_eq!(erc20.allowance(AccountId::from([0x0; 32]),100), Ok(()));
            assert_eq!(erc20.allowanceof(AccountId::from([0x1; 32]), AccountId::from([0x0; 32])), 100);
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut erc20 = Erc20::new(999);
            assert_eq!(erc20.totalsupply(), 999);
            assert_eq!(erc20.allowance(AccountId::from([0x1; 32]),200), Ok(()));
            assert_eq!(erc20.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 100), Ok(()));
        }

        
        
    }
}