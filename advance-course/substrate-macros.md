# Substrate 宏的介绍

## 内容
- Rust宏
- Runtime常用的宏
- cargo expand
- 其他宏

## Rust 宏

宏 Macro 是一种**元编程**的方式，常见的还有Java里的反射，Rust提供了两种宏：
- 声明宏
- 过程宏

[the book ch19 06 macros](https://doc.rust-lang.org/book/ch19-06-macros.html)


## Substrate 为什么使用宏

为了简化Runtime的开发，Substrate使用宏建立了一套DSL(Domain Specific Language), 设计合理的DSL可以：
- 很好的被用户理解
- 代码更加简洁，提升效率
- 解放应用开发者，只需实现业务组件
- 通过使用宏可以使应用开发者不用去考虑，数据库、点对点网络、密码学等底层逻辑只需关注业务逻辑即可。（数据库、点对点网络、密码学、WASM执行环境， 。。。)
- DSL特定领域语言

## Substrate Runtime 定义

链上的状态从一个状态到另一个状态之间的转换，是通过交易完成的，也就是runtime逻辑实现的。
从state1 到state2 是通过Runtime逻辑实现状态之间的转换的。通过交易驱动runtime的逻辑转换，进而实现状态的转移。

内置的模块也成为Pallet 调色板 

- Substrate Module 
    - assert
    - babe
    - balances
    - collective
    - contract
    - democracy
    - elections
    - grandpa
    - indices
    - grandpa
    - indices
    - membership
    - offences
    - session
    - staking
    - sudo 
    - system
    - timestamp
    - treasury
    - and more ... 
- 分类
    - 资产管理: asserts, balances
    - 共识: babe, grandpa
    - 治理: collective, democracy, treasury, 

## Runtime 模块的组成

使用Substrte 进行Runtime模块开发的过程中，常用到的宏有：
- decl_storage 定义存储单元, 链上存储单元
- decl_module 包含可调用函数, 每一个可调用函数对一种类型
- decl_event 事件 
- decl_error 错误信息
- construct_runtime 添加模块到Runtime， 将自己编写的模块添加到Runtime 中去

## decl_storage 

不管是web2.0传统的互联网应用，还是区块链技术的web3.0应用，关键数据都需要存起来。

decl_storage宏，就是用来定义runtime模块的存储单元。

## decl_storage 例子

```rust
/// The pallet's configuration trait. 
pub trait Trait: system::Trait {
    /// The overaching event type. 
    type Event: From<Event<Self>> + Into<<Self as system::Trati>::Event>;
}

// This pallet's storage items. 
decl_storage!{
    triat Store for Module<T: Trait> as TemplateModule {
        Something get(fn something): Option<u32>; // 单值类型，映射类型，双射类型
    }
}
```

## decl_module 

区块链的链上状态变化由**交易触发**，Substrate 不仅支持自定义的存储数据结构，还支持自定义的交易，例如**转账、注册身份、投票**等等、也叫做extrinsic外部交易。

decl_module用来定义模块里可调用函数，每一个外部交易都会触发一个**可调用函数**，并根据交易体信息也就是**函数参数**，更新链上状态。


## decl_module 例子

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin { // 这里的Origin是来自system::Trait中的Origin
        type Error = Error<T>;
        fn deposit_event() = default; // 保留函数

        #[weight = 10_000] 
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
            // --snip--
            Something::put(something);
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }

        #[weight = 10_000]
        pub fn cause_error(origin) -> dispatch::DispatchResult {
            
            // --snip-- 
            match Something::get() {
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?,
                    Something::put(new);
                    Ok(())
                },
            }
        }
    }
}
```

Runtime 模块里存在**保留函数**，除了deposit_event之外，还有：

- on_initialize, 在每个区块的开头执行；定时器的队列里面有没有一个待执行的任务，然后执行该任务
- on_finalize, 在每个区块结束时执行；
- offchain_worker: 开头且是链外执行，不占用链上的资源；链外工作机，计算复杂度比较高，需要与链外资源交互
- on_runtime_upgrade: 当有runtime升级时才会执行，用来迁移数据。

## decl_event 

区块链是一个**异步系统**，runtime 通过**触发事件**通知交易执行结果。

```rust
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId { // AccountId 是来自frame_system中的AcccountId
        SomethingStored(u32, AccountId),
    }
)

Self::deposit_event(RawEvent::SomethingStored(something, who));// who是发送交易方
```

## decl_error 

```rust
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Value was None
        NoneValue, // Index 0
        /// Value reached maximum and cannot be incremented further
        StorageOverflow, // index 1
    } 
}
```

可调用函数里的错误类型，
- 不能给他们添加数据
- 通过metadata 暴露给客户端；
- 错误发生时触发system.ExtrinsicFailed事件，包含了对应错误的信息。


## contruct_runtime 加载模块

```rust
// 在加入到runtime 先实现接口
impl template::Triat for Runtime { // Runtime 是由construct_runtime的宏生成的
    type Event = Event;
}

construct_runtime!(
    pub enum Runtime where 
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // snip
        TemplateModule: template::{ Module, Call, Storage, Event<T>, Error}, // 这里的引用顺序是有关系的，有相关依赖关系的
    }
)

```

## cargo expand 

将宏里的代码展开，得到Rust的标准语法。

https://github.com/dtolnay/cargo-expand
https://github.com/kaichaosun/play-substrate/blob/master/pallets/template/expanded.rs


## 其他宏


decl_runtime_apis & impl_runtime_apis,定义runtime api: 

https://substrate.dev/recipes/3-entrees/runtime-api.html
https://substrate.dev/rustdocs/master/sp_api/macro.decl_runtime_apis.html
https://crates.parity.io/sp_api/macro.impl_runtime_apis.html

runtime_interface, 定义在runtime 里可以调用的Host提供的函数：

https://substrate.dev/rustdocs/v2.0.0-alpha.8/sp_runtime_interface/attr.runtime_interface.html

解释：将runtime_api中定义的接口暴露给客户端使用

runtime - host，当需要runtime 中的函数需要调用host的函数时，需要使用runtime_interface, 

runtime类比为蛋黄，host类比为蛋白





