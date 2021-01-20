# Substrate Kitties (1)

> junus@parity.io

## 课程大纲 Substrate Kitties教程 

- Metadata 元数据介绍
- 模块功能开发
- 单元测试
- FRAME资产相关模块介绍
  - balances
  - assets
- 作业

## 作业

编程作业，需要完成以下要求并且提交代码链接：

1. 指出视频中实现kitties的一个bug

2. kittyindex不在pallet中指定，而是在runtime里面绑定

3. 扩展存储，能得到一个账号拥有的所有kitties

4. 设计一个好的数据结构使得

   1. 能得到一个kitty的parents, brothers, children.

      以及和它一起breed过的另一半

   2. 分析时间空间复杂度，尽量使得操作的较为高效

   Kitties

   KittiesCount

   KittyOwners

   KittyTotal

   KittiesParents

   KittiesChidren

   KittiesBrother

   KittiesPartner

5. 测试代码能检查event, 能测试所有的三个方法，能测试出所有定义的错误类型

6. create和breed需要质押一定数量的token，在transfer的时候能转移质押。

时间：

2021/1/12



## 课程内容

- Metadata 元数据介绍

- Kitties Pallet开发

- Frame资产相关模块

  - Ballances 

  - Assets

## Metadata 元数据

其中包含了每个模块的元数据

这里的使用场景是用来描述runtime模块

Storage

Events

Calls

Constants

Errors

Index 区块中函数的调用有很大关系

动态升级，不同的区块高度的区块中的metadata是不一样的

substrate的官方文档

substrate 元数据

Substrate frame metadata lib.rs

Substrate frame supprot metadata.rs

## Balance 

存储token 数量

账户拥有的数量

transfer

锁定资产

查询资产

balance对单一资产管理的模块

管理多个资产需要多次实例化balance

```
**Existential Deposit:** The minimum balance required to create or keep an account open. This prevents
"dust accounts" from filling storage. When the free plus the reserved balance (i.e. the total balance)
fall below this, then the account is said to be dead; and it loses its functionality as well as any
prior history and all information on it is removed from the chain's state.
No account should ever have a total balance that is strictly between 0 and the existential
deposit (exclusive). If this ever happens, it indicates either a bug in this module or an
erroneous raw mutation of storage.
```

```
Reserved Balance
```

```
Lock 多次lock重复使用
```

```
Existential Deposit
```



# Substrate Kitties 教程 (2)

- 链上升级和数据迁移
- 模块间功能复用
- 模块功能开发，单元测试，UI
- FRAME 治理相关模块介绍
  - sudo
  - democracy
  - collective
  - treasury
  - elections
  - phragmen
  - membership
- 作业

## 作业

前端基于kitties-cource 已有前端 （https://github.com/SubstrateCourse/kitties-hw）加以下UX及功能，这部分共10分：

- 能创建一个毛孩
- 每一个毛孩展示成一张卡片，并显示是不是属于你的
- 可以转让毛孩给另一个用户

## Substrate Kitties (3)

## 课程内容

- 链上升级和数据迁移
- Pallet功能复用
- UI开发
- Frame治理相关模块
  - sudo
  - membership
  - collective
  - treasury
  - Elections-phragmen
  - democracy



## 3.1 链上升级

### 3.1.1 为什么substrate 能升级（指的是链上逻辑的升级）

substrate 把runtime都编译成WASM， 并保存在链上， Client读取WASM代码，通过WASM Executor 来进行状态转变，当新的WASM代码设置到链上之后，新的runtime逻辑就生效了。

📒：需要注意的是这里的升级指的是指runtime 升级，链上逻辑的升级。过去的以前都是将底层的组件与runtime一起编译在一起生成二进制文件（普通的区块链系统是将底层的代码（client）和runtime编译在一起, 因此如果runtime升级了整个系统都需要去升级，不同节点需要去做协调，达到节点的一致性。）。substrte对于这个做了一个新的技术上的突破，采用将链上的逻辑功能编译成WASM文件，升级后的WASM文件被上传到链上达到无分叉升级的功能，链上提供了一个WASM的虚拟机，这样就只可以变更生成的runtime的WASM文件，这样就可以达到了链上升级的功能。

从两个角度来看：从一个角度来看，runtime在底层上就是一份数据，更改runtime逻辑就是更新这一份数据，也可以看做是一个很大的智能合约（substrate runtime 就相当于是一个很大的智能合约），代码逻辑和数据做了分离。

Substrate的技术新特性，跨链了， 链上升级，功能复用，模块的自由的一个组合。

### 3.1.2 升级的过程

（1）升级spec版本号。

（2）编译新的WASM， **WASM_TARGET_DIRECTORY=$(pwd) **（用来设置存放WASM文件的，这个WASM文件就是runtime编译之后的文件，也就是链上逻辑的文件）。普通的编译会将wasm文件放在一个很隐秘的地方。

（3）通过Sudo或者链上治理来更新WASM。对于升级来说在测试链上 来说很容易，通过sudo来实现，到生产环境就要通过链上治理来实现。

polkadot.js 交易 -- sudo -- system -> setCode + file 可能出现错误，警报说invalid transaction 超过了block限制的weight就会报错， 在setCode 是可能出错。

要使用uncheckWeight 这个方法， 就不会对这个weight去做检查。

## 数据迁移



### substrate 链上数据存储

所有的数据都是放在一个key， value的数据库中。

原始数据的key: twox128(module_prefix) ++ Twox128(storage_prefix)

Map类型数据的Key: Twox128(module_prefix) ++ Twox128(storage_prefix) ++ hasher(encode(key))











