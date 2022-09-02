# Substrate Recipes-Zh

为有抱负的区块链工程师准备的实践指南

Substrate Recipes 是一本使用的工作示例指南，演示了在使用substrate建立blockchain的最佳做法。每个食谱中包含了完整的工作代码，以及详细的解释代码。

## 如何使用这本书

你可以按照任何特定的顺序去读这本书。如果你有一个特定话题想要深入研究，或者想要寻找主题/关键字，请使用搜索🔍按钮（左上角的小放大镜）来搜索主题。这个列表是按照不断增加的复杂性大致排列的。

你不能仅仅通过阅读来学习建立blockchain, 在你研究recipe的过程中，练习编译，测试和修改每个recipe，玩耍这些代码，提取其中的模式，并且将他们应用于你想要解决的问题！

如果你还没有，你可能应该立即克隆这个存储库。

```
git clone https://github.com/substrate-developer-hub/recipes.git
```

## 寻求帮助

在学习任何新技能时，你都会不可避免地在某个时刻陷入困境，当你陷入困境时，你可以通过以下几种方式寻求帮助：

- 在[stack overflow](https://stackoverflow.com/questions/tagged/substrate)上请求帮助
- 在[substrate Technical Element channel](https://app.element.io/#/room/#substrate-technical:matrix.org) 上询问帮助
- 给这个仓库[提交issue](https://github.com/substrate-developer-hub/recipes/issues/new)

## 先决条件

每个recipe针对的是substrate开发的一个特定内容的细节，在所有的recipe有一些基本substrate开发和rust环境的假设。一般而言，你应该满足已下先决条件：

- 有一个工作的substrate 开发环境，这是[官方文档的设置substate开发环境的指南](https://substrate.dev/docs/en/knowledgebase/getting-started/)
- 理解[The Rust Book 官方图书](https://doc.rust-lang.org/book/index.html)的前十章的内容，你应该是边学习substrate便学习rust而不是在学习substrate之前先学习rust
- 完成几个[官方的substrate的教程](https://substrate.dev/en/tutorials)

## Substrate节点的结构

认识编码的抽象是很有用的

为了理解存储库中的代码是如何组织的, 我们首先来看一下substrate节点是如何构造的。每个节点都有许多组件来管理，例如 事务队列 (transaction queue)，通过p2p网络进行通信，就区块链的状态和链的实际运行时逻辑达成共识。节点的每个方面本身都很有趣的，运行时是特别有趣的，因为它包含了业务逻辑（状态转换函数），这些业务逻辑造就了链的功能



很多，但并不是所有的recipes都专注于使用FRAME, Parity的框架由称为pallet的单个模块组成的，使用FRAME 构建的运行时通常包含这样的pallet, Kitchen node是你要遵守的范例在你创建你自己的blockchain之前。

![substrate-architecture](../pic/substrate-architecture.png)

## 我们的Kitchen 目录

如果你还没，那么现在就可以克隆它。

- Consensus: 用于底层节点的一致性引擎
- Node: 准备运行的完整的substrate 节点
- Pallets: pallets 用于FRAME-based的运行时
- Runtimes: 用于底层节点的运行时
- Text: 这本书的来源，也就是你现在正在读的东西

探索这些目录就会发现这样的一棵树

```
recipes
|
+-- consensus
  |
  +-- manual-seal
  |
  +-- sha3pow
|
+-- nodes
    |
    +-- basic-pow
    |
    +-- ...
    |
    +-- rpc-node
|
+-- pallets
    |
    +-- basic-token
    |
    + ...
    |
    +-- weights
|
+-- runtimes
    |
    +-- api-runtime
    |
    + ...
    |
    +-- weight-fee-runtime
|
+-- text

```

## Kitchen Node 的内部

让我们深入的了解一下Kitchen Node 

查看Kitchen Node 的Cargo.toml文件，我们看到它有 许多的依赖。它们中的大多数来自substrate自身，事实上，Kitchen Node的大部分并不是独一无二的或者专门的，并且substrate提供我了我们可以使用的健壮的实现，运行时不来自substrate，相反，我们使用runtime文件夹中的super-runtime。

Node/kitchen-node/Cargo.toml

```
# This node is compatible with any of the runtimes below
# ---
# Common runtime configured with most Recipes pallets.
runtime = { package = "super-runtime", path = "../../runtimes/super-runtime" }

# Runtime with custom weight and fee calculation.
# runtime = { package = "weight-fee-runtime", path = "../../runtimes/weight-fee-runtime"}

# Runtime with off-chain worker enabled.
# To use this runtime, compile the node with `ocw` feature enabled,
#   `cargo build --release --features ocw`.
# runtime = { package = "ocw-runtime", path = "../../runtimes/ocw-runtime" }

# Runtime with custom runtime-api (custom API only used in rpc-node)
# runtime = { package = "api-runtime", path = "../../runtimes/api-runtime" }
# ---

```

上面引用的注释表明super runtime不是我们选择的唯一运行时，我还可以使用weight-free 运行时，我鼓励你尝试这个实验（请记住，在前面的部分重新编译节点的指令）

每个节点都必须又有一个运行时。你可以通过查看我们Kitchen中包含的其他节点的Cargo.toml文件来确认这一点

## Super Runtime 的内部

了解了kitchen Node依赖于运行时之后，现在我们更深入地了解Super runtime 

Runtimes/super-runtime/cargo.toml

```
# -- snip --

# Substrate Pallets
balances = { package = 'pallet-balances', , ... }
transaction-payment = { package = 'pallet-transaction-payment', ,... }
# Recipe Pallets
adding-machine = { path = "../../pallets/adding-machine", default-features = false }
basic-token = { path = "../../pallets/basic-token", default-features = false }

```

这里我们看到运行时依赖于许多pallet，这些pallet中的一些来自substrate自身。事实上，substrate提供了一个丰富的常用pallet集合，你可以在你自己的runtime中使用这些pallet， runtime还可以包含一些自定义的pallet，写在我们的kitchen Node中

## 常见模式

我们刚刚观察了这个recipe使用的一般模式，从内到外，我们看到一些pallet代码存储在pallet/<pallet-name>/src/lib.rs  通过在runtimes/<runtime-name>/Cargo.toml中添加pallet的名称和相对路径，将pallet包含到runtime中，然后在node/<node-name>/Cargo.toml中添加运行时的名称和相对路径，将其安装到节点中。

一些recipe探索了运行时之外的blockchain 开放方面的东西，回顾本节开头的节点体系结构，可以想象更改节点的RPC或者Consensus在概念上类似于更改其运行时。


