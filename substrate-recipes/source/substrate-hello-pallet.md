# Pallets 

Pallet是在FRAME运行时使用的单独的运行时逻辑块。在recipe的这一部分你将了解他们。

## 1.1 Hello Substrate 

Pallets/hello-substrate 

这个pallet有一个可调度的调用，可以将消息打印到节点输出。打印到节点日志在运行时并不常见，但是在调试和作为"hello world"示例时非常有用，因为这是recipes中的第一个pallet，所以我们还要看看pallet 的一般结构

### No std

代码的第一行告诉rust编译器，除非显示地告诉它，否则这个crate不应该使用rust的标准库，这是有用的，因为在substrate运行时编译到WebAssembly中，标准库不可用。

```
#![cfg_attr(not(feature = "std"), no_std)]
```

## Imports

接下来，你讲找到来自substrate框架的各个部分的导入，所有的pallet将import一些公共的crate，包括f[rame-support](https://substrate.dev/rustdocs/v2.0.0/frame_support/index.html), [frame-system](https://substrate.dev/rustdocs/v2.0.0/frame_system/index.html). 复杂pallet将有许多import， hellp-substrate pallet 的imort有

```rust
use frame_support::{ decl_module, dispatch::DispatchResult, debug };
use frame_system::{ self as system, ensure_signed };
use sp_runtime::print;
```

## Test

接下来，我们看到对测试模块的引用，这个pallet和大多数的recipes pallet一样，有一个名为test.rs的单独的文件中编写了测试

## Configuration Trait 

接下来，每个pallet都有一个configuration triat 称为Trăit, 配置trait可用于方位其他pallet的特性，或者影响pallet行为的[常量](https://substrate.dev/recipes/constants.html)。这个pallet足够简单，我们的配置trait可以保持为空，尽管他必须存在。

```rust
pub trait Trait: system::Trait {}
```

## Dispatchable Calls

一个Dispatch call 是一个函数，blockchain用户可以作为一个外部的一部分调用，"Extrinsic"是substrate的术语，意思是来自链外部的调用。大多数的时候，他们都是transaction,目前把它们看作transaction是可以的，Dispatchable call是在[decl_moudle!](https://substrate.dev/rustdocs/v2.0.0/frame_support/macro.decl_module.html) 宏中定义的。

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        /// A function that says hello to the user by printing messages to the node log
        #[weight = 10_000]
        pub fn say_hello(origin) -> DispatchResult {
            // --snip--
        }

        // More dispatchable calls could go here
    }
}

```

正如你可以看到的，我们的hello-substrate pallet有一个dispatchable call，有一个参数称为origin, 这个调用返回[DispatchResul](https://substrate.dev/rustdocs/v2.0.0/frame_support/dispatch/type.DispatchResult.html)t，他可以是Ok(())表示调用成功，或者是Err，表示失败，在大多数的pallet都有演示。

## Weight Annotations

在看hello-substrate的函数之前，我们看到一行#[weight = 10_000]，这一行调用赋予了一个默认的权重。最终权重会影响用户调用函数所需支付的费用，权重是使用substrate 开发的一个非常有趣的方面，但是他们也在稍后的[权重](https://substrate.dev/recipes/weights.html)一节中讨论，现在许多recipes pallet ， 都将简单地使用默认的weight。

## Inside a Dispatchable Call

让我们仔细看看我们的dispatchable call 

```rust
pub fn say_hello(origin) -> DispatchResult {
    // Ensure that the caller is a regular keypair account
    let caller = ensure_signed(origin)?;

    // Print a message
    print("Hello World");
    // Inspecting variables
    debug::info!("Request sent by: {:?}", caller);

    // Indicate that this call succeeded
    Ok(())
}

```

这个函数主要做了三件事，首先，他使用[ensure_signed](https://substrate.dev/rustdocs/v2.0.0/frame_system/fn.ensure_signed.html) 函数确保函数的调用者是拥有私钥的普通用户，这个函数还返回调用者是谁，我们将调用者的标示村村在callet变量中。

其次，它输出一条消息并记录调用者，注意，我们没有使用rust的正常的println!宏，而是一个特殊的[print](https://substrate.dev/rustdocs/v2.0.0/sp_runtime/fn.print.html)函数和[debug::info!](https://substrate.dev/rustdocs/v2.0.0/frame_support/debug/macro.info.html) 宏，原因将在下一节中解释。

最后，调用返回Ok(()). 表示调用成功，咋一看，似乎没有办法让这个调用失败，但事实并非如此，如果调用不是来自一个签名的origin，则在开头的ensure_sined函数可以返回一个错误。这是我们第一次看到“前验证，后写入”的重要范例，在substrate 开发中，重要的是始终确保先决条件得到满足并在开始时返回错误。在这些检查完成之后，你可以开始函数的计算。

## Printing From the Runtime 

打印到终端从rust程序中通常是非常简单的时候println!宏，但是，substrate 运行时被编译为WebAssembly和常规的二进制文件，并且不能访问rust的标准库，这意味着我们不能使用常规的println! 我们鼓励你修改代码，尝试使用println并确认他不会编译，尽管如此，从运行时打印消息对于记录信息和调试都是有用的。

![test](/Users/suyinrong/bitcoin-proj/substrate-course/substrate-recipes/pic/substrate-architecture.png)

在我们的pallet顶部，我们导入了sp_runtime运行时[print](https://substrate.dev/rustdocs/v2.0.0/sp_runtime/fn.print.html)函数，这个特殊的函数允许运行时想节点外部传递一条打印消息，这条消息没有被编译到wasm，并且可以访问标准库并执行常规IO，这个函数只能打印实现可[Printable triat](https://substrate.dev/rustdocs/v2.0.0/sp_runtime/traits/trait.Printable.html)的item. 幸运的是，所有的基本类型都已经实现了这个trait， 你也可以Wie自己的数据类型实现这个trait。

打印函数 not： 要实际打印的消息，我们需要在运行kitchen node 时使用标记-lruntime=debug，因此，命令变为了

./target/release/kitchen-node --dev -lruntime=debug

下一行演示了如何使用debug::info! 宏将日志显示在屏幕上，并检查变量的内容，宏的语法非常类似于常规rust宏println



Runtime logger note" 当我们以本机方式执行运行时，debug::info!信息是打印出来的，而如果我们在wasm 执行运行时，我们还需要一个额外的步骤来初始化[RuntimeLogger](https://substrate.dev/rustdocs/v2.0.0/frame_support/debug/struct.RuntimeLogger.html) 