# Runtime  APIs

pallets/sum-storage 

Runtimes/api-runtime

每个Substrate节点都包含了一个runtime, runtime包含链的业务逻辑，它定义了哪些事务是有效的和无效的，并确定链的状态在响应事务时如何变化。runtime被编译成Wasm, 以方便runtime升级，除了runtime之外，“外部节点’不会编译到Wasm, 只会编译到本机。外部节点负责处理对等发现，事务池，块和事务传送，一致性以及回复外部RPC调用。在执行这些任务时，外部节点有时需要向运行时查询信息，或者向运行时提供信息。运行时API促进了外部节点和运行时之间的这种通信。在这个recipes中，我们将编写自己的最小运行时API。

## Our Exampe 

对于这个例子，我们将编写一个称为sum-storage 的pallet， 其中包含两个存储项(u32类型)

```rust

decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        Thing1 get(fn thing1): Option<u32>;
        Thing2 get(fn thing2): Option<u32>;
    }
}
```

substrate 已经提供了一个用于查询存储值的运行时API,这就是为什么我们可以很容易地从前端查询两个存储值，在这个例子中，我们假设外部节点对两个值的和感兴趣，而不是对单个值感兴趣。我们的runtime API将为外部节点提供一种方法来查询运行时获得这个两个值sum值， 在定义实际的runtime API之前，让我们在pallet中编写一个公共的帮助函数用来求和。

```rust
impl<T: Trait> Module<T> {
    pub fn get_sum() -> u32 {
        Thing1::get() + Thing2::get()
    }
}
```

到目前为止，我们所做的一切都不是针对运行时api的，在接下来的章节中，我们将在运行时API的实现中使用这个help函数。

## Defining the API

向runtime添加runtime APIs 的第一步是使用Rust trait定义其接口，这在sum-storage/runtime-api/src/lib.rs文件中完成，这个文件可以存在于任何你喜欢的地方，但是因为他定义了一个与特定pallet密切相关的API，所以讲API定义包含在pallet的目录中是有意义的。

定义API的diamante非常简单，看起来几乎想任何旧的Rust triat,另外，它必须放置在decl_runtime_apis! 宏中，这个宏允许外部节点在特定块处查询运行时API。虽然这个运行时API只提供一个函数，但是你可以编写任意多的函数。

```rust
sp_api::decl_runtime_apis! {
    pub trait SumStorageApi {
        fn get_sum() -> u32;
    }
}
```

## Implementing the API 

通过编写pallet和定义运行时API，现在可以实现运行时API，这法伤在runtime 聚合文件中，在本例中我们在runtimes/apis-runtimes/src/lib.rs提供了api-runtime。

与定义API一样，实现一个运行时API看起来类似于实现任何旧的Rust trait，只是实现必须在impl_runtime_apis! 宏的内部。每个运行时都必须使用impl_runtime_api! 因为核心API是必需的，我们将在这个宏中为我们自己的API和其他API一起添加一个实现，我们的实现是直接，因为它仅仅调用我们前面编写的pallet辅助函数。

```rust
impl_runtime_apis! {
  // --snip--

  impl sum_storage_rpc_runtime_api::SumStorageApi<Block> for Runtime {
        fn get_sum() -> u32 {
            SumStorage::get_sum()
        }
    }
}
```

你可能想知道这里提到的Block类型参数，但是在我们的定义中没有。这个类型参数是由宏和其他一些特性一起添加的，所有运行时APIs都有这个类型参数，以便方便在任意块处查询运行时，在impl_runtime_api文档中了解更多相关信息。

## Calling the Runtime API

我们现在已经成功的为运行时添加了一个运行时API，外部节点现在可以调用这个API来查询运行时中两个存储值之和。给出一个客户端的参考，我们可以这样调用。

```rust
let sum_at_block_fifty = client.runtime_api().get_sum(&50);
```

这个recipe是关于定义和实现一个自定义的运行时API，要查看在实践中调用这个API的示例，请参见custom RPCs上的recipes，其中我们将这个运行时API连接到一个可由终端用户调用的RPC。

