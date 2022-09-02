

# Substrate ink！ 智能合约开发

> Smart Contract 教程
- 基于Substrate 的智能合约介绍
    - 模块pallet-contract, pallet-evm介绍 
    - ink!, Solidity介绍
    - transactional storage
- 使用ink!编写智能合约
- 社交相关模块介绍
    - identity
    - recovery
    - society
- 作业

## Substrate

Built using Rust and Wasm.

## Why WebAssembly?

Wasm is a platform independent executable format
- Wasm is Sandboxed 
- Wasm is Fast
- Wasm is Compact
- Wasm is Well Supported

## Why Rust?

Parity is Rust company as much as it is a blockchain company.
- Rust builds directly to Wasm
- Rust has a rapidly developing ecosystem
- Rust code is more ergonomic and easier to get right!
- You technically don't need to use Rust.

## 合约模块
It's not magic, but feels magical.

## 合约的架构

- 存储`#[ink(storage)]`
- 合约实例化方法`#[ink(constructor)]`
- **公共方法（用户可调用）**`#[ink(message)]`
- 事件`#[ink(event)]`

## 环境初始化设置
https://substrate.dev/substrate-contracts-workshop/#/0/setup

```shell
rustup component add rust-src --toolchain nightly 
rustup target add wasm32-unknown-unknown --toolchain stable

# 安装canvas 节点
cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.4 --force --locked

# 安装contract 插件
cargo install cargo-contract --vers 0.8.0 --force --locked

```

## 实战

cargo contact new erc20
// erc20 合约
```rust
#[cfg_attr(not(features = "std"), no_std)]

use ink_lang as ink;

#[ink::contract] // (env = DefaultEnviroment)
mod erc20 {

     #[ink(event)]
     pub struct Transfer {
         #[ink(topic)]
         from: AccountId,
         #[ink(topic)]
         to: AccountId,
         value: Balance,
     }
    

    
    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InSufficientBalance,
        InSufficientAllowence,
    }

    type Error<T> = core::result::Result<T, Error>;

    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        // 定义的总的供给量
        total_supply: Balance,
        // 定义账户对应的余额
        balances : StorageHashMap<AccountId, Balance>,
        // 允许A账户向b账户转账的余额
        allowance: SotrageHashMap<(AccountId, AccountId), Balance>,

    }

    impl Erc20 {
        // 构造函数
        #[ink(constructor)] 
        pub fn new(total_supply: Balance) -> Self {
            // 这笔合约初始化的发起人
            let caller = Self::env().caller();

            let mut balances = StroageHashMap::new();
            
            balances.insert(caller, total_supply);

            Self {
                total_supply,
                balances,
                allowance: StorageHashMap::new(),
            }
        }

        // 定义的 获得总供给量的函数
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        // 定义的获得某个账户有多少余额的函数
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }
        
        // 定义的获得owner可以下个spender转账的余额
        #[ink(message)]
        pub fn allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: balance) -> Result<()> {
            let from_allowance = self.allowance_of(from, to);
            if from_allowance < value  {
                return Err(Error::InSufficientBalance);
            }

            self.allowance.insert((from, to), from_allowance - value);
            
            self.transfer_help(from, to, value)?;

            Ok(())
        }

        // #[ink(message)]
        // pub fn burn() {

        // }

        // ##[ink(message)]
        // pub fn issue() {}


        #[ink(message)]
        pub fn transfer(&mut self, to: Accountd, value: Balance) -> Result<()> {
            let caller = Self::env().caller();
            self.transfer_help(caller, to, value)?;
            Ok(())
        }

        // 定义的私有函数
        fn transfer_help(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            
            // 要想从from的账户发送出去 需要检查from账户的余额是否足够
            let from_balance = self.balance_of(from);

            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            
            // 对from 的balance余额减少
            self.balances.insert(from, from_balance - value);

            let to_balance = self.balance_of(to);

            // 对to的balance余额增加
            self.balances.insert(to, to_balance + value);

            // 定义发送事件
            self.env().emit_event(Transfer {
                from,
                to,
                value,
            });

            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        
        use super::*;

        #[test]
        fn create_contract_works() {
            let erc20 = Erc20::new(1000);
            assert_eq!(erc10.total_supply(), 1000);
        }

    }

    
}

```

## 调用

```shell
./target/release/substrate --dev --tmp
```
polkadot.js/app



可以通过一份代码创建很多数目的合约

## 工具推荐

https://polkadot.js.org/apps

https://paritytech.github.io/canvas-ui/

https://patract.io/


## 示例代码

https://github.com/paritytech/ink/blob/master/examples/erc20/lib.rs




## 和合约调用runtime的方法



需要两端都做实现

https://github.com/paritytech/ink/pull/645/files



