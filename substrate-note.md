# Substrate 区块链开发入门



> Https://substrate.dev



## 导师

孙凯超 kaichao@parity.io

Maggie Done maggiedong@parity.io

Jimmy Chu jimmy.chu@parity.io



## 资料

- [Substrate 官方文档](https://substrate.dev) 
- Rust [官方网站](https://www.rust-lang.org/)和[官方书籍](https://doc.rust-lang.org/book/)
- 博客文章 [如何学习Substrate](https://zhuanlan.zhihu.com/p/161771205) 



## 什么是区块链



### 传统记账方式

所有的账本都存在银行的中心化服务器 



### 区块链的记账方式

每个人都可以保存一份账本



### 随之而来的问题

谁可以往账本上写东西？

- 如何证明你是你？
- 如何防止恶意攻击？

如何确保所有人维护的账本都一样？

- 为什么要维护同一份账本？
- 如果有人的账本和其他的人不一样，会怎么办？

回答： 

1. 谁可以向账本里写入内容呢？ 是任何人都可以写入

2. 当有人伪造一个账户向账本中写入数据，应该返回错误。因为只有本人可以操纵自己的账户，所以每当发起交易时，必须向所有人证明你是你 。

3. 怎么证明你是你呢？ 数字签名算法

4. 传统的签名方法，签名不会随着内容的变更，签名发生变化，但是数字签名会随着签名的内容而变化

5. 一般还会在每笔交易的前面有nonce值，就算数字的签名的内容是一样的但是nonce值是不一样的。也不会成功

6. 伪造转账的记录暂时告一段落了，但是如何保证所有人都维护同一份账本呢？通过在每个交易的前面加上一个nonce值

   

### 签名和验证

```
Sign(secret_key, message) -> Signature
Verify(public_key, signature, message) -> true/false
```



### 小结

1. 账本 =  交易历史
2. 每个人都可以向账本里添加内容
3. 用私钥签名，公钥向其他人证明。因为私钥绝对不可以泄露，但是公钥可以展示给别人
4. 每个人的交易里包含了一个自增长数字(nonce)， 用于防止别人复制攻击





每个人都可以保存账本， 如何确保每个人的账本都一样呢？

如何维护账本的一致性：通过共识机制



### 什么是Hash (Trapdoor dunction)

将信息加密后的消息得到的是0/1的比特串，但是不能由加密后的串得到原始的信息

通常需要将交易记录还有加上一个nonce值做hash运算得到0/1的比特串



### 什么是工作量证明PoW(Proof of Work)

求解nonce的过程， 求解一个nonce使得求得后的字符串的前多少位为0. 

特点是 难计算。易证明



### 什么是区块

交易的记录加上nonce值



### 什么是区块链 

将区块通过hash值作为指针连接起来的链称为区块链



### 真实的区块是一个什么样子

一个区块要包含区块头还有区块体



### 为什么大家都要维护一样的账本 

有奖励



### 小结

哈希函数；将任何文件 文本变成一串256bit的01串，难碰撞，不可逆

工作量证明 => 不停地算出复合要求的哈希值

产生分叉怎么办 => 等待最长链的出现



## Polkadot 介绍



### 互联网的发展看区块链未来 跨链

- Web 1.0 只读互联网
- Web 2.0 可读写互联网
- Web 3.0 价值互联网



### 区块链发展现状

- 公链
  - 特性：开发复杂，乌托邦，生态丰富，维护成本高，升级困难，开发，犯错成本高
  - Defi, DAO, Staking, Economics, Oracle, ZK, Consensus, Crypto, 跨链， TEE
- 联盟链
  - 特性：CA准入，开发简单，开发框架单一，功能单一，升级困难，可扩展性低，生态单调
  - 业务



### 波卡是什么？

波卡要把不同的区块链连接到一起

愿景：一个跨链协作的未来

波卡在解决下面三个问题：

- 跨链协作
- 可扩展性
- 共享安全



### 为什么跨链是未来

- 区块链可以解决信任问题
- 专业化可以解决可扩展问题
- 可交互可以在可扩展的前提下，解决更广泛的信任问题，是一个更真实的世界



### 传统区块链根据场景做不同的取舍

- Scalability
- Security 
- Decentralization



### 波卡尝试打通整个区块链生态沟通协作



### 也因为能沟通协作，波卡也允许无上限扩展



### Polkadot 的组件

- Relay Chain 中介链
- Parachain 平行链
- Validators 区块验证节点
- Collators 区块整理节
- Bridge 桥



### Polkadot 与Substrate 



### 可交互的联盟链 数据共享 全局信任

- 政务链
- 银行链
- 供应链
- 资产管理链
- 人才链
- 征信链
- 核心链

### 联盟链式趋势，公链是未来



## Substrate 介绍



### Substrate -- 从Polkador 提取的开发经验



### Substrate 是构造区块链的框架



### 区块链包含哪些组件

- 数据库
- 点对点网络
- 共识算法
- 交易处理
- 状态转换函数(Runtime)
- 其他特别的函数，零知识证明(ZK), 分片

### 区块链的构造组件

- 区块链的基础核心组件
  - 数据库
  - 加密算法
  - 点对点网络
  - 序列化
  - 共识算法
  - 交易队列
- 治理升级模块
  - 链逻辑升级
  - 链上治理
- dapp
  - 智能合约



### Substrate特点

- 模块化
- 开源
- 自主可控



### Substrate 具体包含些什么？

- 核心模块
  - 数据库
  - 交易队列
  - 命令行界面
  - 公密钥生成
  - RPC
- 基本逻辑
  - 数据库
  - 结算
  - 时间戳

- p2p网络
  - 网络节点管理
  - 私讯协议继承
  - 分布式哈希表

- 共识机制
  - 抵押
  - Babe
  - Grandpa
  - 区块落实追踪
- 链上治理
  - 民主
  - 投票
  - 议会
  - 国库



### Polkadot Substrate 之上建立的模块

- 平行链 Parachains 
- 区块整理 Collators 
- 跨链通讯协定 Cross Chain Message Passing 
- 私讯协议 Gossip Protocol
- 持续可用存储 Persistent Availability Store
- 平行线程 Parathreads 
- 众筹模块 Crowd Funding
- 赔偿 Claims
- 拍卖 Auctions
- 公证 Registrar 



### The Substrate Runtime 

Runtime 是区块链的链上运行逻辑的集合，也就是状态转换函数

Runtime ： 资产 共识 余额  Collective  合约 治理 选举 grandpa 账户 块确认 indices 会员 offences session 抵押 超级权限 system 时间戳 国库 and more ... 



### 一键链上升级 - 永不分叉

判断native runtime和链上的runtime版本是否一致 选择是执行wasm runtime 还是 native runtime 



### Runtime 升级的治理Governing Runtime Upgrades 

- Runtime 代码可以通过链上治理访问
- Sudo 模块
- Democracy 模块
- 自定义的模块和逻辑
- Runtime 升级和可选的



### 为什么需要链上升级

- 修复重要的安全漏洞
- 改变核心规则
- 添加新功能
- 修复链上状态
- 硬分叉需要的协作成本极高，易升级失败
- 没有明确的治理策略和升级时间点

使用Wasm，升级过程无需节点直接参与，如果不适用Wasm，整个网络都需要执行升级操作

### Substrate 与企业系统无缝集成

使区块链成为解决方案的一分部分

- 使用内置的链下工作机，提供与SAP, Oracle, SQL 服务器以及更多其他企业系统的无缝集成支持
- 支持集成Trusted Execution Environments(TEEs)
- 内置与区块链预言机的双向集成
- 使用JSON-RPC代理的中间件集成



### Substrate 是公链技术 生态和联盟链之间的桥梁

通过Substrate 分享最先进的区块链技术成果



### Building a Hub for Developers 

Https://substrate.io

Tutorials, videos, playground, community, recipes, docs, samples



### Substrate 中文学习资料

- Substrate.io 

- Bilibili https://zhuanlan.zhihu.com/p/161771205
- 知乎 https://zhuanlan.zhihu.com/substrate https://zhuanlan.zhihu.com/v2web3 
- 微信公众号 polkdaot 中文平台 Polkaworld Polkabase



### 如何运行substrate节点

- 单节点 （dev)
- 多节点（local）



# Substrate node Template



### Polkadot JS Apps 

- Generalized and Hosted UI
- Quickly test new functionality
- Loaded with general tools like:
  - Creating transactions
  - Read stroage 
  - See events
  - And way more ... 
- Great for development 



### 模块定义概览

```rust
use support::{decl_module, decl_storage, decl_event, ... };
pub trait Trait: system::Trait { ... }

decl_storage! { ... }
decl_event! { ... }
decl_error! { ... }
decl_module! { ... }
impl<T: Trait> Module<T> { ... }
```



### 引入和定义关联类型

```rust
pub trait Trait: system::Trait {
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// .. and it inherits from system::Trait
// From 'system' pallet 
pub trait Trait : 'static + Eq + Clone {
  type Origin: ... 
  type Call: ...
  type Index: ...
  type BlockNumber: ...
}
```



### 定义存储

```rust
decl_storage! {
  trait Store for Module<T: Trait> as TemplateModule {
    // Here we are declaring a StorageValue, 'Something' as an Option<u32> 
    // 'get(fn something)' defines a getter function 
    // Getter called with 'Self::thing()'
    Something get(fn something) : Option<u32>;
    // Here we are declaring a StorageMap 'SomeMap' from an AccountId to a Hash 
    // Getter called with 'Self::some_map(account_id)'
    SomeMap get(fn some_map) : map hasher(identity) T::AccountId => u32;
  }
};
```



### 定义事件

```rust
decl_event! {
  pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
    /// Event 'SomethingStored' is declared with a parameter of the type 'u32' an 'AccountId'
    SomthingStored(u32, AccountId),
  }
};
```



### 定义可调用函数

```rust
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn deposit_event<T>() = default; // The default deposit_event definition 
    
    pub fn do_something(origin, something: u32) -> Result {
      let sender = ensure_signed(origin)?; // Check for transaction
      Something::put(something); // put a value into a StoreageValue 
      Self::deposit_event(RawEvent::SomethingStored(somthing, who)); // Emit Event 
      Ok(()) // Return Ok at the end of a function 
    }
  }
}
```



### 定义公共和私有函数

```rust
impl<T: Trait> Module<T> {
  fn mint(to: T::AccountId, id: T::Hash) -> Result { ... }
  fn transder(from: T::AccountId, to: T::AccountId, id: T::Hash) -> Reuslt {
    /// ... 
  }
}

// 如果定义为'pub'其他模块也可以调用
```



### Coding 

https://github.com/SubstrateCourse/substrate-node-template



### 帮助连接

- https://www.substrate.io/

- Substrate: Node Template,  https://github.com/substrate-developer-hub/substrate-node-template, Front-end Template https://github.com/substrate-developer-hub/substrate-front-end-template
- Offical: Substrate Github https://github.com/paritytech/substrate Polkador Github https://github.com/paritytech/polkadot
- 官方中文 知乎专栏 https://zhuanlan.zhihu.com/substrate ，bilibili  https://space.bilibili.com/67358318
- Rust: Rust programming book  https://doc.rust-lang.org/book/
- Rust reference : https://doc.rust-lang.org/stable/reference/introduction.html



### 如何参与开源项目

- Star & Watch & fork
- Check issues and PRs regularly 
- Join riot/discord chammels
- Feel free to ask any questions, stupid ones are always welcomed
- Follow the instructions to contribute 



## Rust 入门导学



### 课程目的

- 了解Rust语言的主要特点
- 学习Rust语言的基本类型和基本运算符
- 学习Rust语言的控制结构
- 了解Rust语言的字符串
- 学习Rust语言的枚举与模式匹配
- 学习Rust语言的Result与Option
- 学习Rust语言的错误处理
- 了解Rust语言的所有权与生命周期的概念
- 学习Rust语言的Trait
- 学习Rust语言的泛型 
- 学习Rust语言的迭代器
- 学习Rust语言的宏



### Rust 语言的主要特点

- 高性能
- 内存安全
- 无忧并发

#### 高性能

- 与C/C++ 一个级别的运行速度
- 方法学习
  - zero Cost Abstratct 领开销抽象
  - 无GC的自动内存管理RAII
  - 可做到C ABI 一致性的设计

#### 内存安全

- 使用Rust写出的代码， 保证内存安全
- 方法抉择
  - Owbership, move 语义
  - Borrow checker
  - 强类型系统
  - 无空值(Null, nill)设计

#### 无忧并发

- 使用Rust进行多线程以及多任务并发代码开发，不会出现数据竞争和临界值破坏
- 方法抉择
  - 对并发进行了抽象 Sync, Send
  - 融入类型系统
  - 基于Ownership, Borrow checker实现 完美的融合性

### 基础数据类型 

#### 整数 浮点数

#### 布尔类型

#### 字符char

#### 元组Tuple

#### 数组 Array 

#### 函数代码块

#### if 表达式

#### 无条件循环

#### while循环

#### for循环

### 字符串

#### &str 

#### String

#### More

### Slice

### 复合类型 

#### 结构体

##### 结构体的初始化和字段更新

##### 元组结构体（匿名字段结构体）

##### 裸结构体

#### 枚举

##### 枚举的基本使用

##### 类C的枚举

##### 强大表现力的枚举

##### 对等表示

##### 模式匹配

### Result && Option 

#### Rust 中无空值

#### Error Handling

#### 错误处理示例



### Ownership && Borrowing 

### 所有权示例

### 生命周期

### 生命周期管理的实现

### trait

### 定义一个trait

### 为一个结构体实现一个trait

### 使用triat中实现的方法

### Trait Bound 特征限定

### Generics泛型

### 两个泛型示例

### Iterator 迭代器

### Std 标准库定义的Iterator Trait 

### 实现一个自己的迭代器

### 使用我们自己的迭代器

### 宏

### substrate中的宏 声明宏

### substrate中的宏 过程宏



## 创建第一条区块链应用



### 课程内容

- 环境安装
- 代码编译
- 启动多节点本地区块链
- Node Template 代码导读
- 开发一个ERC20的Pallet



### 编译和运行环境的安装(ubuntu)



#### 1. 安装依赖的库

sudo apt install -y cmake pkg-confifig libssl-dev git gcc build-essential clang libclang-dev llvm curl



#### 2. 安装Rust语言

curl https://sh.rustup.rs -sSf | sh

rustup update nightly

rustup target add wasm32-unknown-unknown --toolchain nightly 

rustup update



### 下载并编译代码



#### 下载Substrate 代码 并编译

git clone https://github.com/paritytech/substrate

cargo +nightly build --release



#### IDE (VS code , Clion)



### 运行一个单节点的区块链



#### 使用编译出来的可执行程序，启动链

./target/release/substrate --dev



#### 子命令

purge-chain 清除所有的链上数据

build-spec 生成spec.json文件



#### 日志的设置

export RUST_LOG=info



#### 参考链接

https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/



 ### 启动参数

--base-path 数据存放路径 

--chain 指定使用的链的类型

--alice 使用预先定义的密钥 

--port p2p通信的端口

--ws-port websocket服务端口

--rpc-port rpc服务端口

--node-key 指定libp2p使用的私钥

--telemetry-url 统计数据提交的地址 

--validator 作为验证人加入网络

--light 运行轻客户端模式 



### 运行两个节点的区块链



#### 使用local模式来启动两个节点，来组成一条链



#### 观察日志中出块和节点链接情况



#### 参考链接

https://substrate.dev/docs/en/tutorials/start-a-private-network/

```
 // Start Alice's node
 ./target/release/substrate --base-path /tmp/alice \
 --chain local --alice \
 --port 30333 --ws-port 9944 --rpc-port 9933 \
 --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
 --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
 --validator
 
 // Bob join the network
 ./target/release/substrate --base-path /tmp/bob \
 --chain local --bob \
 --port 30334 --ws-port 9945 --rpc-port 9934 \
 --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
 --validator \
 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
 
```



### Note Template 代码导读



### Node template 代码分析



#### Code base 

#### https://github.com/SubstrateCourse/substrate-node-template



#### 知乎文章关于Node Template 

https://zhuanlan.zhihu.com/p/123167097



### 基于Template 开发ERC20

### ERC20规范

https://eips.ethereum.org/EIPS/eip-20

#### 编程实现一个ERC20的pallet



## Polkadot-JS App介绍



### 内容

- 安装及运行
- 与节点连接
- 查询及交易



### Polkadot-JS App 与 Polkadot-JS API 

- Polkadot-JS App
  - 官方的与Substrate互动的前端应用
  - 可让你专注做区块链开发而先不用自行搭建前端
- Polkadot-JS API 
  - 与Substrate 互动的官方JS API 



### 安装及运行

- 使用网上版本

  访问：https://polkadot.js.org/apps/#/explorer

- 在本机

  git clone https://github.com/polkadot-js/apps

  cd apps

  yarn install

  yarn start



### 跑起开发节点node-template

git clone https://github.com/substrate-developer-hub/substrate-node-template

cd substrate-node-template

cargo build --release 

target/release/node-template purge-chain --dev

target/release/node-template --dev



### 连接节点

- 点解上方网络名字Kusama 或者logo

- 点选你要的连接的节点，最㡳下有本机开发节点  (http://127.0.0.1:9944)  或选 custom endpoint 输入节点 domain 或 IP 或端口。默

  认 Substrate websocket 端口是 9944。

### 查询及交易

- 节点资料

  - 区块链资料
  - 区块链近期发出的事件

- 查看个人账号 (包括browser extension 内的) 及余额(Accounts)

- 作基本余额转移(Transfer) (1 unit = 1 * 10^12)

- 链上查询(chain state)

  - 例子：查詢现在的链上时间：timestamp > now() 就是调用 timestamp module/pallet 的 now() 这个函数 

- 交易 (Extrinsics)

  - 例子：余额转移： balances > transfer

- 超管操作

- 设置(setting)

  - 语言

  - 自定义类型(Custom Type) 

    当你在 Substrate 节点内定义你自己的类型，API 是不知道的，那就要你告 诉 Polkadot-JS api, 

    你的自定义类型，及最后连接到什么原始类型。

    文档 ： https://polkadot.js.org/docs/api/



### 查询及交易 - custrom type

```
{
  "Address": "AccountId",
  "LookupSource": "AccountId"
}
```



## Polkadot-JS API 



### 内容

- 介绍
- 创建接口 及 调试
- 链上查询及大数值bn.js
- 查看常量及远程过程调用
- 交易(Extrinsics)
- 拾取用户账号
- 订阅事件
- 自定义类型



### 介绍

- 官方的与基于 Substrate 框架开发的区块链 API

- 可用来查询链上状态，对交易作签名，提交交易(具签名及不具签名的)，订阅链

  上事件等。

- 用了 JS Promise 的写法。因此也需要熟悉用 js 里 async / await 的语法 

- API 里与链互动的函数是根据链上的元数据 (meta-data) 动态生成。

  例子：你要有一个 templateModule pallet 的 doSomething() 函数在 Substrate 

  内, 才在 JS api 里有 api.tx.templateModule.doSomething() 这函数。



### 开始使用

官方文档 https://polkadot.js.org/api/ 

开始使用：

\# 如果只是与 Substrate 节点交互

yarn add @polkadot/api

yarn add @polkadot/keyring



\# 如果是在浏覧器内互前端交互，最好还加

yarn add @polkadot/extension-dapp

yarn add @polkadot/ui-keyring

yarn add @polkadot/ui-settings



### 材料

Git clon 这两个项目

- Substrate node template 
- Node template 内的templateModule 及 doSomething()



### 创建接口

然后第一件事就是创建一个 api 对象并连到远端接口

```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';

// 指定远端接口
const wsProvider = new WsProvider('ws://127.0.0.1:9944');

// 创建接口
const api = await ApiPromise.create({ provider: wsProvider });

// 简单测试
console.log(api.genesisHash.toHex());


```

### 调试Substrate API

在 polkadot-JS App 内 Javascript 一栏，在代码里加

Debugger;

跑的时候，就可以在 console 内看到有什么函数可以调用



### 链上查询

Api.query. < pallet >.< storage >

ex: 

```javascript
const val = await api.query.templateModule.something();
```

这是对应着连接的 Substrate 节点，有 templateModule, 并在

decl_storage!{...} 内定义了 something 的 getter 函数

```javascript
const val = await api.query.templateModule.something();
```

这里返回的是一个 对象，接着可以用: 

● val.isSome / val.isNone - 看是不是有值 

● val.toJSON() - 查看其值 

● val.toString() - 变成字符串

● val.toHuman() - 显示用户友好的值



### 大数值bn.js

- 很多区块链的数值都是大于 JS 本身 Number 的值 2^53  \- 1, Polkadot-JS API 用了

  bn.js 来包装它。比如：

```javascript
const acct = await api.query.system
 .account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
const freeBalance = acct.data.free;
debugger;
```

freeBalance 是一个 bn 对象，可以用 freeBalance.toString(10) 看它十进制的值，以字付串显示。



### 查看常量

以 api.consts.<pallet>.<constant> 来获取。比如：

```javascript
const minPeriod = api.consts.timestamp.minimumPeriod.toString();
```

因为常量在连接到节点时已取得，不是动态查询，所以不是异步操作，不需要用 await。



### 远程过程调用RPC

api.rpc.<callable>.<function> 比如：

```javascript
const hash = await api.rpc.chain.getBlockHash();
console.log(hash.toString());
```

这里 `hash` 这个对象可以用 `toString()` 来查看最新出块的哈希值。你也可传入一个数 block number, 看到对应区块数的哈希值。



### 交易(Extrinsics) -- 具签名

要作交易，主要用到"Api.tx.*"的

```javascript
const hash = await api.tx.templateModule.doSomething(22)
 .signAndSend(alice)
```

- 用 alice 這個用户，发出了一个具签名的交易
- 对应用有 templateModule pallet 并在 decl_module!{...} 有定义doSomething() 的函数。



### 交易(Extrinsics ) -- 不具签名



```javascript
(async () => {
const hash = await api.tx.templateModule.someFunction(22)
 .send()
})();
```

- 注意 Substrate 节点默认不处理不具签名的交易
- 在 Substrate 节点得定义不具签名交易的处理函数，这交易才能顺利处 理

### 撷取用户帐号 

- 不建议用户在应用内创建钱包帐号。创建钱包帐号可以透过polkadot-JS extension 来进行，支援 Chrome 及 Firefox 浏覧器。就像

Metamask

- 当 Substrate 节点以开发模式 (--dev)运行时，它是会自动生成 `Alice`, `Bob`, `Charles`, `Dave`, `Eve` 的帐号。

所以我们用以下方法要拿取这两类不同 (dev 及 extension )的帐号

```javascript
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';
import keyring from '@polkadot/ui-keyring';

// 允许 app 可读取 polkadot-js extension 管理的帐号
await web3Enable('My DAPP');

// 遍历polkadot-js extension 管理的帐号
let allAccounts = await web3Accounts();
allAccounts = allAccounts.map(({ address, meta }) =>
 ({ address, meta: { ...meta, name: `${meta.name} (${meta.source})` } }));

// 载入包括开发节点的帐号
keyring.loadAll({ isDevelopment: true }, allAccounts);
```

跟着以 keyring.getPairs() 就能遍历你能取得的帐号。然后 alice 就是

```javascript
const alice = keyring.getPairs()[0];
```



### 订阅事件

在 api.query. *,  api.rpc. *  可在最后一个参数放入一个订阅函数:

```javascript
const unsub = await api.query.system.account(alice, acct => {
// 每当这帐号资讯变更，订阅函数会被呼叫一次
});
...
unsub();
```

返回一个取消订阅事件的函数

在 api.tx.*

```javascript
// 假设 alice 已经存储着一个用户帐号
const unsub = await api.tx.templateModule.doSomething(22).signAndSend(alice, result 
=> {
const { status, events } = result;
// ...
}).catch(err => {
// ...
});
```

在 `sendAndSend()` 或 `send()` 后再输入一个函数参数，这就是一个订阅函数，可知道交易的最新状况。里面有：

● `status` 可知道是否导入了区块里 `isInBlock`，或已最后确认`isFinalized`. 

● `events` 可知道这交易触发的事件。

● `.catch()` 里面的就是错误处理。

● 返回一个取消订阅的函数。



### 增加自定义类型

- 如在 Substrate 开发自己的模块增加了自定义类型 (新的 `type` 或`struct`), 那在开发前端也需要告知 api 这类型。

- 方法是当你在一开始创建 api 接口时，再传入一个 types 的参数

```javascript
const api = await ApiPromise.create({
 provider: wsProvider,
 types: {
 Balance: 'u64'
 }
});
```



## Runtime宏介绍



### 内容

- Rust宏
- Runtime 常用的宏
- cargo expand 
- 其他宏



### Rust 宏

宏（Macro）是一种元编程的方式，常见的还有Java里的反射，Rust提供了两种宏：

- 声明宏
- 过程宏

https://doc.rust-lang.org/book/ch19-06-macros.html



### Substrate 为什么使用宏

为了简化 Runtime 的开发，Substrate 使用宏建立了一套 DSL (Domain Specifific Language)，设计合理的DSL可以：

● 很好的被用户理解

● 代码更加简洁，提升效率

● 解放应用开发者，只需实现业务组件



### Substrate Runtime 定义

State A ----交易&Runtime逻辑 ---> State B 

内置的模块也称为Pallet(调色板)



#### Substrate Module 

Assets, babe, balances, collective, contract, democracy, elections, grandpa, indices, membership, offences, session, staking, sudo, system. Timestamp, treasury, 



### Runtime 模块的组成

使用Substratet进行 Runtime 模块开发的过程中，常用到的宏有：

● decl_storage 定义存储单元 

● decl_module 包含可调用函数

● decl_event 事件

● decl_error 错误信息

● construct_runtime 添加模块到 Runtime



### decl_storage 

不管是 web2.0 传统的互联网应用，还是采用区块链技术的 web3.0 应用，关键数据都需要存起来。

decl_storage宏，就是用来定义 runtime 模块的 存储单元。



#### decl_storage 例子

```rust
/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
// This pallet's storage items.
decl_storage! {
  trait Store for Module<T: Trait> as TemplateModule {
    Something get(fn something): Option<u32>;
	} 
}
```



### decl_module 

区块链的链上状态变化由交易触发，Substrate 不仅支持自定义的存储数据结构，还支持自定义的交易，例如转 账、注册身份、投票等等，也叫做 extrinsic 外部交易。

decl_module 用来定义模块里可调用函数，每一个外部交易都会触发一个可调用函数，并根据交易体信息也就是函数参数，更新链上状态。

```rust
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;
		
    #[weight = 10_000]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// -- snip --
			Something::put(something);
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}
    
    #[weight = 10_000]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			// -- snip --
			match Something::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Something::put(new);
					Ok(())
			},
		} 
 	} 
}
```

Runtime 模块里存在保留函数，除了 deposit_event 之外，还有：

● on_initialize，在每个区块的开头执行；

● on_fifinalize，在每个区块结束时执行；

● offchain_worker：开头且是链外执行，不占用链上的资源；

● on_runtime_upgrade：当有 runtime 升级时才会执行，用来迁移数据。



### decl_event

区块链是一个异步系统，runtime 通过触发事件通知交易执行结果。

```rust
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		SomethingStored(u32, AccountId),
 	}
);

Self::deposit_event(RawEvent::SomethingStored(something, who));
```



### decl_error

```rust
decl_error! {
  pub enum Error for Module<T: Trait> {
     /// Value was None
     NoneValue,
     /// Value reached maximum and cannot be incremented further
     StorageOverflow,
  }
}
```

可调用函数里的错误类型，

● 不能给它们添加数据；

● 通过 metadata 暴露给客户端；

● 错误发生时触发system.ExtrinsicFailed 事件，包含了对应错误的信息。



### construct_runtime 加载模块

```rust

impl template::Trait for Runtime {
 type Event = Event;
}

construct_runtime!(
    pub enum Runtime where
      Block = Block,
      NodeBlock = opaque::Block,
      UncheckedExtrinsic = UncheckedExtrinsic
      {
      // -- snip --
      TemplateModule: template::{Module, Call, Storage, Event<T>},
      }
);
```



### cargo expand

将宏里的代码展开，得到 Rust 的标准语法。

https://github.com/dtolnay/cargo-expand

https://github.com/kaichaosun/play-substrate/blob/master/pallets/template/expanded.rs



### 其它宏

decl_runtime_apis & imp_runtime_apis，定义runtime api：

https://substrate.dev/recipes/3-entrees/runtime-api.html

https://substrate.dev/rustdocs/master/sp_api/macro.decl_runtime_apis.html

https://substrate.dev/rustdocs/master/sp_api/macro.impl_runtime_apis.html

runtime_interface，定义在 runtime 里可以调用的 Host 提供的函数：

https://substrate.dev/rustdocs/v2.0.0-alpha.8/sp_runtime_interface/attr.runtime_interface.html



### 多实例模块

Substrate 的模块在 runtime 里可以有多个实例，例如，可以添加多个内置的 collective 模块实例，分别用来表示理事会和技术委员会，来实现复杂的治理模型。

```rust
pub trait Trait<I: Instance = DefaultInstance>: frame_system::Trait {
	type Event: From<Event<Self, I>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
  trait Store for Module<T: Trait<I>, I: Instance=DefaultInstance> as Collective {
  // -- snip --
  } 
}

decl_module! {
  pub struct Module<T: Trait<I>, I: Instance = DefaultInstance> for enum Call 
    where origin: T::Origin {
  // -- snip --
  }
}


decl_event!(
  pub enum Event<T, I: Instance = DefaultInstance> where
   <T as frame_system::Trait>::AccountId,
  {
  // -- snip --
  }
);
decl_error! {
  pub enum Error for Module<T: Trait<I>, I: Instance> {
  // -- snip --
  } 
}


type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Trait<CouncilCollective> for Runtime {
  // -- snip --
}
construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = node_primitives::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
     {
    Council: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, 
    Config<T>},
   }
);
```



## Runtime 数据存储的设计



### 内容

● 区块链存储的不同点和约束 

● Substrate 存储单元的类型 

● 存储的初始化

● 最佳实践



### 区块链存储的不同点



#### 区块链应用通常几个特点，

● 开源可审查，对等节点，引入延迟和随机来达到共识 

● 链式、增量地存储数据



#### 区块链应用的节点软件依赖高效的键值对数据库： 

● LevelDB

● RocksDB



### 区块链应用通常几个特点，

● 开源可审查，对等节点，引入延迟和随机来达到共识 

● 链式、增量地存储数据

区块链应用的节点软件依赖高效的键值对数据库： 

● LevelDB

● RocksDB



### 区块链存储的约束

区块链作为业务的载体，存储相关的限制有：

● 大文件直接存储在链上的成本很高；

● 链式的区块存储结构不利于对历史数据的索引； 

● 另外一个约束是，在进行数值运算时不能使用浮点数。



### Substrate 存储单元的类型 

开发链上存储单元的特点：

● Rust原生数据类型的子集，定义在核心库和alloc库中 

● 原生类型构成的映射类型 

● 满足一定的编解码条件



### 单值类型 

存储某种单一类型的值，如布尔，数值，枚举，结构体等：

● 数值：u8,i8,u32,i32,u64,i64,u128,i128

● 大整数：U128,U256,U512

● 布尔：bool

● 集合：Vec<T>, BTreeMap, BTreeSet

● 定点小数：Percent,Permill,Perbill

● 定长哈希：H128,H256,H512

● 其它复杂类型：Option<T>,tuple,enum,struct

● 内置自定义类型：Moment,AccountId



#### 数值类型 u8,i8,u32,i32,u64,i64,u128,i128 的定义

```rust
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
      // store unsigned integer, init to zero if not set
      MyUnsignedNumber get(fn unsigned_number): u8 = 10;
      // also init to zero, can store negative number
      MySignedNumber get(fn signed_number): i8;
   }
}
```

#### 数值类型 u8,i8,u32,i32,u64,i64,u128,i128 的使用：

● 增：MyUnsignedNumber::put(number);

● 查：MyUnsignedNumber::get();

● 改：MyUnsignedNumber::mutate(|v| v + 1);

● 删：MyUnsignedNumber::kill();

更多API，请参考文档 Trait frame_support::storage::StorageValue



#### 数值类型 u8,i8,u32,i32,u64,i64,u128,i128 的安全操作：

● 返回Result类型：checked_add, checked_sub, checked_mul, checked_div

// fail the transaction if error

my_unsigned_num.checked_add(10)?; 

● 溢出返回饱和值：saturating_add,saturating_sub,saturating_mul

// result is 255 for u8

my_unsigned_num.saturating_add(10000); 



### 大整数 U256,U512 类型定义：

```rust
use sp_core::U256;
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    // init to 0
   	MyBigInteger get(fn my_big_integer): U256;
  }
}
```

操作：checked_add,overflowing_mul... 

更多API，参考文档 sp_core::U256



### bool 类型定义：

```rust

decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    	// init to false, store boolean value
     MyBool get(fn my_bool): bool;
   }
}
```



#### Vec<T> 类型定义：

```rust
use sp_std::prelude::*;
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    // default to 0x00
     MyString get(fn my_string): Vec<u8>;
   }
}
```

操作：push,pop,iter… Vec结构体



### Percent,Permill,Perbill 类型定义：

```rust
use sr_primitives::Permill;
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    // fix point number, default to 0
     MyPermill get(fn my_permill): Permill;
   }
}
```



### Percent,Permill,Perbill 类型操作：

- 构造
  - Permill::from_percent(value);
  - Permill::from_parts(value);
  - Permill::from_rational_approximation(p,q);

- 计算
  - permill_one.saturating_mul(permill_two);
  - my_permill * 20000 as u32

API文档 sp_arithmetic::Permill



### Moment 时间类型定义：

```rust
pub trait Trait: system::Trait + timestamp::Trait {}
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    // Moment is type alias of u64
     MyTime get(fn my_time): T::Moment;
   }
}
```

获取链上时间：<timestamp::Module<T>>::get();



### AccountId 账户类型定义：

```rust
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
    MyAccountId get(fn my_account_id): T::AccountId;
  }
}
```

获取AccountId: let sender = ensure_signed(origin)?;



### struct 类型定义：

```rust
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default)]
pub struct People {
 name: Vec<u8>,
 age: u8,
}
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
   	MyStruct get(fn my_struct): People;
  }
}
```



### enum 类似，还需要实现Default接口

https://github.com/kaichaosun/play-substrate/blob/master/pallets/datatype/src/lib.rs#L32



### 简单映射类型

#### map 类型，用来保存键值对，单值类型都可以用作key或者value，定义：

```rust
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
   	MyMap get(fn my_map): map hasher(twox_64_concat) u8 => Vec<u8>;
  }
}
```

hasher: blake2_128_concat, twox_64_concat, identity



#### map 类型，用来保存键值对，单值类型都可以用作key或者value，用法：

- 插入一个元素：MyMap::insert(key, value);
- 通过key获取value：MyMap::get(key);
- 删除某个key对应的元素：MyMap::remove(key);
- 覆盖或者修改某个key对应的元素
  - MyMap::insert(key, new_value);
  - MyMap::mutate(key, |old_value| old_value+1);

API 文档：Trait frame_support::storage::StorageMap 

​					Trait frame_support::storage::IterableStorageMap



### 双键映射类型

double_map 类型，使用两个key来索引value，用于快㏿删除key1对应的任意记录，也可以遍历key1对应的所有记录，定义：

```rust
decl_storage! {
  trait Store for Module<T: Trait> as DataTypeModule {
     MyDoubleMap get(fn my_double_map): double_map hasher 
    (blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => 
    Vec<u8>;
   }
}
```

double_map 类型，使用两个key来索引value，用于快㏿删除key1对应的任意

记录，也可以遍历key1对应的所有记录，定义： 

- 插入一个元素：MyDoubleMap::<T>::insert(key1, key2, value);
- 获取某一元素：MyDoubleMap::<T>::get(key1, key2);
- 删除某一元素：MyDoubleMap::<T>::remove(key1, key2);
- 删除 key1 对应的所有元素：MyDoubleMap::<T>::remove_prefix(key1);

API文档 frame_support::storage::StorageDoubleMap

​	Trait frame_support::storage::IterableStorageDoubleMap



### 存储的初始化

创世区块的数据初始化，有三种方式：

● config()

● build(clousure)

● add_extra_genesis { … }



演示: 

https://github.com/kaichaosun/play-substrate/blob/master/pallets/genesis-confifig/src/lib.rs



### 最佳实践 

- 最小化链上存储 
  - 哈希值 
  - 设置列表容量
- Verify First, Write Last



### 其它Tips

- 可以通过pub关键字设置存储单元的可见范围 
- 可以手动设置默认值，如: 
  - MyUnsignedNumber get(fn unsigned_number): u8 = 10;
- 在frame目录下查找对应的最新用法
-  decl_storage 宏的说明文档



## 存证模块的功能开发



### 内容

- 存证的介绍 
- 写代码 
- 作业



### 链上存证的介绍 

存证是一种在线服务，可用于在某一时间点验证计算机文件的存在性，最

早是通过比特币网络带有时间戳的交易实现的。存证的应用场景有：

● 数字版权 

● 司法存证 

● 供应链溯源

● 电子发票 



### 链上存证的功能实现

https://substrate.dev/docs/en/tutorials/build-a-dapp/



### 作业

第一题：列出 个常用的宏、 个常用的存储数据结构；

第二题：实现存证模块的功能，包括：

● 创建存证； 

● 撤销存证。

第三题：为存证模块添加新的功能，

● 转移存证，接收两个参数，一个是内容的哈希值，另一个是存证的

接收账户地址。