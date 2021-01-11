# Transaction Weight 

Resources available to chains are limited. The resources include memory usage, storage I/O, computation, transaction/block size and state database size. There are several mechanisms to manage access to resources and to prevent individual components of the chain from consuming too much of any resource. Weights are the mechanism used to manage the *time it takes to validate* a block. Generally speaking, this comes from limiting the storage I/O and computation.

链的可用资源是有限的。资源包括内存使用、存储I/O、计算、事务/块大小和状态数据库大小。有几种机制来管理对资源的访问，并防止链的各个组件消耗过多的任何资源。权重是用来管理验证一个区块所需时间的机制。一般来说，这来自于限制存储I/O和计算。

NOTE: Weights are not used to restrict access to other resources, such as storage itself or memory footprint. Other mechanisms must be used for this.

注意：权重不用于限制对其他资源的访问，如存储本身或内存占用。必须使用其他机制来实现这一点。

The amount of weight a block may contain is limited, and optional weight consumption (i.e. weight that is not required to be deployed as part of the block's initialization or finalization phases nor used in mandatory inherent extrinsics) will generally be limited through economic measures --- or in simple terms, through transaction fees. The fee implications of the weight system are covered in the [Transaction Fees document](https://substrate.dev/docs/en/knowledgebase/runtime/fees).

一个区块可能包含的权重数量是有限的，可选的权重消耗（即不需要作为区块初始化或最终确定阶段的一部分部署的权重，也不用于强制性的内在外在因素）一般会通过经济措施来限制------或者简单地说，通过交易费用来限制。权重系统的费用影响在交易费用文件中有所阐述。

Substrate defines one unit of weight as one picosecond of execution time on fixed reference hardware (Intel Core i7-7700K CPU with 64GB of RAM and an NVMe SSD). Benchmarking on reference hardware makes weights comparable across runtimes, which allows composability of software components from different sources. In order to tune a runtime for different validator hardware assumptions, you can set a different maximum block weight. For example, in order to allow validators to participate that are only half as fast as the reference machine, the maximum block weight should be half of the default, keeping the default block time.

Substrate将一个权重单位定义为固定参考硬件（英特尔酷睿i7-7700K CPU，64GB内存和NVMe SSD）上的一皮秒执行时间。在参考硬件上进行基准测试，使得不同运行时的权重具有可比性，这使得来自不同来源的软件组件具有可合成性。为了针对不同的验证器硬件假设调整运行时，您可以设置不同的最大块权重。例如，为了允许速度只有参考机一半的验证机参与，最大块权重应该是默认值的一半，保持默认块时间。

The maximum block weight should be equivalent to one-third of the target block time, allocating one third for block construction, one third for network propagation, and one third for import and verification. Doubling the block time would allow a doubling of the maximum block weight. These tuning options give runtime developers a way to make the optimal transaction per second vs. hardware requirement trade-offs for their use case. These trade-offs can be tuned with runtime updates to keep up with hardware and software improvements.

最大区块权重应相当于目标区块时间的三分之一，分配三分之一用于区块构建，三分之一用于网络传播，三分之一用于导入和验证。将区块时间增加一倍，就可以将最大区块重量增加一倍。这些调优选项为运行时开发者提供了一种方法，可以针对他们的用例进行每秒最佳事务与硬件需求的权衡。这些权衡可以通过运行时更新进行调整，以跟上硬件和软件的改进。

## Weight Fundamentals

Weights represent the *limited* time that your blockchain has to validate a block. This includes computational cycles, and storage I/O. A custom implementation may use complex structures to express this. Substrate weights are simply a [numeric value](https://substrate.dev/rustdocs/v2.0.0/frame_support/weights/type.Weight.html).

权重代表你的区块链验证一个区块的有限时间。这包括计算周期，以及存储I/O。定制的实现可能会使用复杂的结构来表达。substrate权重只是一个数值。

A weight calculation should always:

重量计算应始终。

- Be computable **ahead of dispatch**. A block producer should be able to examine the weight of a dispatchable before actually deciding to accept it or not.

  在dispatch前要有可计算性。区块生产者应该能够在实际决定是否接受一个可派发的货物之前审查其重量。

- Consume few resources itself. It does not make sense to consume similar resources computing a transaction's weight as would be spent to execute it. Thus, weight computation should be much lighter than dispatch.

  本身消耗的资源就很少。计算一个事务的权重所消耗的资源与执行该事务所花费的资源相近是没有意义的。因此，权重计算应该比调度轻得多。

- Be able to determine resources used without consulting on-chain state. Weights are good at representing *fixed* measurements or measurements based solely on the parameters of the dispatchable function where no expensive I/O is necessary. Weights are not so useful when the cost is dependent on the chain-state.

  能够在不咨询链上状态的情况下确定使用的资源。权重擅长于表示*固定的测量或仅基于可调度函数的参数的测量，其中不需要昂贵的I/O。当成本依赖于链上状态时，权重就不那么有用了。

In the case that the weight of a dispatchable is heavily dependent on chain-state case, two options are available:

如果一个可调度产品的重量严重依赖于链态情况，有两种选择

- Determine or introduce a forced upper limit to the amount of weight a dispatchable could possibly take. If the difference between the enforced upper limit and the least possible amount of weight a dispatchable could take is small, then it can just be assumed to always be at the upper limit of the weight without consulting the state. If the difference is too great, however, then the economic cost of making lesser transactions might be too great which will warp the incentives and create inefficiencies in throughput

  确定或引入一个强制上限，规定可调度物资可能承受的重量。如果强制上限与可派遣人员可能承担的最小重量之间的差别不大，那么就可以直接假定其永远处于重量的上限，而不需要征求国家的意见。但如果差别太大，那么进行较少交易的经济成本可能太大，这将扭曲激励机制，造成吞吐量的低效率。

- Require the effective weight (or precursors that can be used to efficiently compute it) be passed in as parameters to the dispatch. The weight charged should be based on these parameters but also cover the amount of time it takes to verify them during dispatch. Verification must take place to ensure the weighing parameters correspond accurately to on-chain state and if they don't then the operation should gracefully error.

  要求将有效权重(或可用于有效计算权重的前兆)作为参数传递给调度。收取的重量应基于这些参数，但也要涵盖调度过程中验证这些参数所需的时间。验证必须进行，以确保称重参数与链上状态准确对应，如果它们不对应，那么操作应该优雅地出错。

### Weight Factors

Several factors impact execution time, and therefore weight calculation. One large contributor is the number of database accesses that are performed by a dispatchable. Because the cost of a database access is greatly dependent on the database backend and storage hardware, the weight calculations are parameterized over the weight costs of database reads and writes. These costs are determined by benchmarking each available database backend on some reference hardware. This allows switching database backends without changing all weight calculations.

有几个因素会影响执行时间，从而影响权重计算。一个大的因素是可调度的数据库访问次数。由于数据库访问的成本在很大程度上取决于数据库后端和存储硬件，因此权重计算的参数是在数据库读写的权重成本上进行的。这些成本是通过在一些参考硬件上对每个可用的数据库后端进行基准测试来确定的。这样可以在不改变所有权重计算的情况下切换数据库后端。

In addition to only using constants for the pre-dispatch weight calculation, the developer has the ability to factor in the input parameters of the given dispatchable. This can be useful when the execution time depends on, for example, the length of one parameter. It is important that these calculations do not entail any meaningful work themselves. The pre-dispatch maximum weight should be trivially computable from the input arguments with some basic arithmetic.

除了只使用常量进行调度前的权重计算外，开发人员还可以将给定的可调度的输入参数考虑进去。当执行时间取决于例如一个参数的长度时，这可能是有用的。重要的是，这些计算本身不需要任何有意义的工作。派遣前的最大权重应该可以通过一些基本的算术从输入参数中琐碎地计算出来。

The [System pallet](https://substrate.dev/rustdocs/v2.0.0/frame_system/struct.Module.html) is responsible for accumulating the weight of each block as it gets executed and making sure that it does not exceed the limit. The [Transaction Payment pallet](https://substrate.dev/rustdocs/v2.0.0/pallet_transaction_payment/index.html) is responsible for interpreting these weights and deducting fees based upon them. The weighing function is part of the runtime so it can be upgraded if needed.

系统托盘负责在每个区块被执行时累积其重量，并确保其不超过限制。交易支付托盘负责解释这些重量，并根据这些重量扣除费用。称重功能是运行时的一部分，因此可以在需要时进行升级。

### Post Dispatch Weight Correction

There are cases where the actual weight of a dispatchable is not trivially computable from its inputs. For example, the weight could depend on the logic path of the dispatchable. Without any means of correcting the weight after dispatch, we would constantly overestimate and subsequently overcharge for those dispatchables as we must assume the worst case ahead of dispatch for the chain to be safe.

在某些情况下，可调度系统的实际权重无法从其输入中琐碎地计算出来。例如，权重可能取决于可调度的逻辑路径。如果没有任何手段来修正调度后的权重，我们就会不断地高估并随后高估这些可调度设备的费用，因为我们必须在调度前假设最坏的情况，以保证链的安全。

The post-dispatch weight correction allows any dispatchable to return its *actual weight* after it was executed. This weight must be less than or equal to the pre-dispatch worst case weight. For a user to be allowed to include an extrinsic, they still must be able to pay for the maximum weight, even though the final payment will be based on the actual weight.

派送后重量修正允许任何可派送物品在执行后返回其*实际重量。该重量必须小于或等于派送前最坏情况下的重量。对于用户来说，如果要允许包含一个外挂物，他们仍然必须能够为最大重量付款，即使最终付款将基于实际重量。

### Block Weight and Length Limit

Aside from affecting fees, the main purpose of the weight system is to prevent a block from being filled with transactions that would take too long to execute. While processing transactions within a block, the System pallet accumulates both the total length of the block (sum of encoded transactions in bytes) and the total weight of the block. If either of these numbers surpass the limits, no further transactions are accepted in that block. These limits are defined in [`MaximumBlockLength`](https://substrate.dev/rustdocs/v2.0.0/frame_system/trait.Trait.html#associatedtype.MaximumBlockLength) and [`MaximumBlockWeight`](https://substrate.dev/rustdocs/v2.0.0/frame_system/trait.Trait.html#associatedtype.MaximumBlockWeight).

除了影响费用外，权重系统的主要目的是防止一个区块被交易填满而导致执行时间过长。在处理一个区块内的交易时，系统托盘会累积该区块的总长度（以字节为单位的编码交易的总和）和该区块的总重量。如果这两个数字中的任何一个超过了限制，那么该块中就不会再接受任何交易。这些限制在 MaximumBlockLength 和 MaximumBlockWeight 中定义。

One important note about these limits is that a portion of them are reserved for the `Operational` dispatch class. This rule applies to both of the limits and the ratio can be found in [`AvailableBlockRatio`](https://substrate.dev/rustdocs/v2.0.0/frame_system/trait.Trait.html#associatedtype.AvailableBlockRatio).

关于这些限制的一个重要说明是，其中有一部分是为操作调度类保留的。这个规则适用于这两个限制，比例可以在AvailableBlockRatio中找到。

For example, if the block length limit is 1 megabyte and the ratio is set to 80%, all transactions can fill the first 800 kilobytes of the block while the last 200 can only be filled by the operational class.

例如，如果区块长度限制为1兆字节，比例设置为80%，那么所有的交易都可以填满区块的前800千字节，而最后200千字节只能由操作类填满。

There is also a `Mandatory` dispatch class that can be used to ensure an extrinsic is always included in a block regardless of its impact on block weight. Please refer to the [Transaction Fees document](https://substrate.dev/docs/en/knowledgebase/runtime/fees) to learn more about the different dispatch classes and when to use them.

还有一个强制性调度类，可以用来确保一个外在的东西总是包含在一个区块中，不管它对区块权重的影响如何。请参考交易费用文档，了解更多关于不同调度类的信息以及何时使用它们。

### Learn More

- [Example](https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs) pallet.
- [Transaction Payment pallet](https://github.com/paritytech/substrate/blob/master/frame/transaction-payment/src/lib.rs)
- [Weights](https://github.com/paritytech/substrate/blob/master/frame/support/src/weights.rs)