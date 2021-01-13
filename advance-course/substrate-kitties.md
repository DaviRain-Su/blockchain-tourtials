# Substrate Kitties 

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

