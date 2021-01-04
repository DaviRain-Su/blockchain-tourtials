# Overview 

Welcome to the wonderful world of blockchain development with Substrate! This is the Substrate Knowledge Base, the official documentation hub for Substrate developers. The purpose of this resource is to help readers understand the multi-disciplinary field of blockchain development with Substrate. This guide is broken down into several sections that explain the principles and design decisions that Substrate is built on as well as the specific skills needed to to be an effective Substrate blockchain developer.

欢迎来到精彩的区块链开发世界与Substrate! 这里是Substrate知识库，是Substrate开发者的官方文档中心。本资源的目的是帮助读者了解使用Substrate进行区块链开发的多学科领域。本指南分为几个部分，解释了Substrate的原则和设计决策，以及成为一个有效的Substrate区块链开发者所需的具体技能。

---

> Some Expertise Needed
>
> In order to get the most out of Substrate, you should have a good knowledge of computer science and basic blockchain concepts. Terminology like header, block, client, hash, transaction and signature should be familiar. Substrate is built on the Rust programming language, which makes use of novel design patterns to enable development of code that is safe and fast. Although you don't need to know Rust to get started with Substrate, a good understanding of Rust will allow you to be a better Substrate developer. Check out the excellent resources provided by the Rust community to build your Rust development skills.
>
> 需要一些专业知识
>
> 为了最大限度地利用Substrate，你应该有良好的计算机科学知识和基本的区块链概念。像header、block、client、hash, transaction和signature这样的术语应该是熟悉的。Substrate是建立在Rust编程语言上的，它利用新颖的设计模式来实现安全快速的代码开发。虽然你不需要懂Rust就能开始使用Substrate，但对Rust的良好理解会让你成为一个更好的Substrate开发者。请查看Rust社区提供的优秀资源，以培养你的Rust开发技能。

Substrate takes a modular approach to blockchain development and defines a rich set of primitives that allows developers to make use of powerful, familiar programming idioms.

Substrate采用模块化的方法进行区块链开发，并定义了一套丰富的primitives，允许开发人员使用强大而熟悉的编程习惯。

## Architecture 

The Substrate client is an application that runs a Substrate-based blockchain node - it consists of several components that include, but are not limited to, the following:

Substrate 客户端是一个运行基于底层的区块链节点的应用程序--它由几个组件组成，包括但不限于以下内容。

- Storage is used to persist the evolving state of the decentralized system represented by a blockchain. The blockchain network allows participants to reach trustless consensus about the state of storage. Substrate ships with a simple and highly efficient key-value storage mechanism

  存储是用来持久保持以区块链为代表的去中心化系统的演变状态。区块链网络允许参与者就存储的状态达成无信任共识。Susbstrate运载了一个简单高效的键值存储机制。

- **Runtime** logic defines how blocks are processed, including state transition logic. In Substrate, runtime code is compiled to [Wasm](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#webassembly-wasm) and becomes part of the blockchain's storage state - this enables one of the defining features of a Substrate-based blockchain: [forkless runtime upgrades](https://substrate.dev/docs/en/knowledgebase/advanced/executor#forkless-runtime-upgrades). Substrate clients may also include a "native runtime" that is compiled for the same platform as the client itself (as opposed to Wasm). The component of the client that dispatches calls to the runtime is known as the [executor](https://substrate.dev/docs/en/knowledgebase/advanced/executor) and it selects between the native code and interpreted Wasm. Although the native runtime may offer a performance advantage, the executor will select to interpret the Wasm runtime if it implements a newer [version](https://substrate.dev/docs/en/knowledgebase/advanced/executor#runtime-versioning).

  运行时逻辑定义了如何处理区块，包括状态转换逻辑。在Substrate中，运行时代码被编译成Wasm，并成为区块链存储状态的一部分--这实现了基于Substrate的区块链的定义特征之一：无叉运行时升级。基底客户端还可能包括一个 "原生运行时"，该运行时是为客户端本身的同一平台编译的（而不是Wasm）。客户端中向运行时调度调用的组件被称为执行器，它在原生代码和解释的Wasm之间进行选择。虽然本地运行时可能提供性能优势，但如果执行器实现了较新的版本，它将选择解释Wasm运行时。

- Peer-to-peer network capabilities allow the client to communicate with other network participants. Substrate uses the libp2p network stack.

  点对点网络功能允许客户端与其他网络参与者进行通信。基底使用libp2p网络栈。

- **Consensus** engines provide logic that allows network participants to agree on the state of the blockchain. Substrate makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been built on top of [Web3 Foundation research](https://w3f-research.readthedocs.io/en/latest/index.html).

  共识引擎提供逻辑，让网络参与者就区块链的状态达成一致。Substrate可以提供定制的共识引擎，同时也提供了几种建立在Web3基金会研究之上的共识机制。

- **RPC** (remote procedure call) capabilities allow blockchain users to interact with the network. Substrate provides HTTP and WebSocket RPC servers.

  RPC（远程过程调用）功能允许区块链用户与网络交互。Substrate提供HTTP和WebSocket RPC服务器。

- Telemetry metrics are exposed by way of an embedded Prometheus server.

  遥测指标是通过嵌入式Prometheus服务器的方式暴露的。

## Usage

Substrate is designed to be used in one of three ways:

Substrate的设计有三种方式之一。

- **With the Substrate Node**: You can run the pre-designed [Substrate Node](https://github.com/paritytech/substrate/tree/master/bin/node) and [configure](https://github.com/paritytech/substrate/blob/master/bin/node/cli/src/chain_spec.rs) its genesis block. In this case, you just need to supply a JSON file and launch your own blockchain. The JSON file allows you to configure the genesis state of the modules that compose the Substrate Node's runtime, such as: Balances, Staking, and Sudo. You can learn more about running a Substrate node in the [Create Your First Substrate Chain](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain) and [Start a Private Network](https://substrate.dev/docs/en/tutorials/start-a-private-network/) tutorials.

  Substrate Node。你可以运行预先设计的Substrate Node，并配置它的genesis block。在这种情况下，你只需要提供一个JSON文件并启动你自己的区块链。JSON文件允许你配置组成Substrate Node运行时的模块的genesis状态，如Balances, Stacking 和Sudo。你可以在《创建你的第一条底层链》和《启动一个私有网络》教程中了解更多关于运行Substrate节点的信息。

- **With Substrate FRAME**: You can easily create your own custom runtime using [FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) (Framework for Runtime Aggregation of Modularized Entities), which is the method used by the Substrate Node. This affords you a large amount of freedom over your blockchain's logic, and allows you to configure data types, select from a library of modules (called "pallets"), and even add your own custom pallets. The [Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template) is a helpful starting point for projects like this. To learn more, see the tutorials to [Build a dApp](https://substrate.dev/docs/en/tutorials/build-a-dapp) and [Add a Pallet](https://substrate.dev/docs/en/tutorials/add-a-pallet).

  Substrate FRAME。你可以使用FRAME（Framework for Runtime Aggregation of Modularized Entities）轻松创建自己的自定义运行时，这也是Substrate Node使用的方法。这为您提供了对您的区块链逻辑的大量自由度，并允许您配置数据类型，从模块库（称为 "pallet"）中选择，甚至添加您自己的自定义pallet。Substrate Developer Hub Node模板是这样的项目的一个有用的起点。要了解更多信息，请参见构建一个dApp和添加一个pallet的教程。

- **With Substrate Core**: The entire FRAME system can be ignored, and the runtime can be designed and implemented from scratch. This could be done in *any language* that can target [WebAssembly](https://webassembly.org/). If the runtime can be made to be compatible with the abstract block authoring logic of the Substrate node, then you can simply construct a new genesis block from your Wasm blob and launch your chain with the existing Rust-based Substrate client. If not, then you will need to alter the client's block authoring logic, and potentially even alter the header and block serialization formats. In terms of development effort, this is by far the most difficult way to use Substrate, but also gives you the most freedom to innovate.

  Substrate心。整个FRAME系统可以被忽略，运行时可以从头开始设计和实现。这可以用任何可以针对WebAssembly的语言来实现。如果运行时可以与Substrate节点的抽象块创作逻辑兼容，那么你可以简单地从你的Wasm blob中构造一个新的genesis块，然后用现有的基于Rust的Substrate客户端启动你的链。如果不是，那么你将需要改变客户端的区块创作逻辑，甚至可能改变头和区块序列化格式。就开发工作而言，这是迄今为止使用Substrate最困难的方式，但也给了你最大的创新自由。

  ## Next Steps

  ### Learn More

  - Refer to the developer documentation for the [FRAME system for runtime development](https://substrate.dev/docs/en/knowledgebase/runtime).

    参考FRAME系统的开发者文档进行运行时开发。

  - Learn how to create rich client applications for any Substrate-based chain by using the [Polkadot-JS](https://substrate.dev/docs/en/knowledgebase/integrate/polkadot-js) family of libraries.

    学习如何使用Polkadot-JS系列库为任何基于Substrate的链创建丰富的客户端应用。

  - Dive deep into advanced topics, like Substrate's [SCALE encoding](https://substrate.dev/docs/en/knowledgebase/advanced/codec), [consensus mechanisms](https://substrate.dev/docs/en/knowledgebase/advanced/consensus), [cryptography](https://substrate.dev/docs/en/knowledgebase/advanced/cryptography), and [storage implementation](https://substrate.dev/docs/en/knowledgebase/advanced/storage).

    深入探讨高级课题，如Substrate的SCALE编码、共识机制、密码学和存储实现。

  ### Examples

  - Follow our [tutorials](https://substrate.dev/tutorials) to learn about building and running blockchains with Substrate and FRAME.

    请跟随我们的教程学习使用Substrate和FRAME构建和运行区块链。

  - Refer to the [Substrate Recipes](https://substrate.dev/recipes/) to find complete working examples that demonstrate solutions to common problems.

    请参阅 "基材配方"，找到完整的工作实例，展示常见问题的解决方案。

  ### References

  - Check out the [Rust reference documentation](https://substrate.dev/rustdocs) that ships with the Substrate code base.

    查看Substrate代码库中的Rust参考文档。

  

  