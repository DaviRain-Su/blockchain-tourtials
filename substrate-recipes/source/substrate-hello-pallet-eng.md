# Pallets 

THE pallet is a separate runtime logical block used at FRAME runtime. In this part of the recipe, you will learn about them.

## 1.1 Hello Substrate 

Pallets/hello-substrate 

This wallet has a schedulable call to print the message to the node output. Printing to node logs is not expected at runtime, but it is very useful for debugging and as an example of "hello world". Because this is the first tablet in recipes, we should also look at the general structure of the tablet.

### No std

The first line of the code tells the trusted compiler that unless it is explicitly told, the crate should not use the trust standard library, which is useful because the standard library is not available when the substitute is compiled into the WebAssembly at runtime.

```
#![cfg_attr(not(feature = "std"), no_std)]
```

## Imports

Next, you will find the imports from various parts of the subordinate framework. All the drawers will import some public crates, including [frame-support](https://substrate.dev/rustdocs/v2.0.0/frame_support/index.html) and [frame-system](https://substrate.dev/rustdocs/v2.0.0/frame_system/index.html). The hello-substrate palette that can be imported is

```rust
use frame_support::{ decl_module, dispatch::DispatchResult, debug };
use frame_system::{ self as system, ensure_signed };
use sp_runtime::print;
```

## Test

Next, we can see the reference to the test module. Like most recipes pallets, this pallet has a separate file named test.rs that has written tests.

## Configuration Trait 

Next, each tablet has a configuration trait called Trăit, configure trait to be used for other wallet features or [constants](https://substrate.dev/recipes/constants.html) that affect the behavior of the wallet. This tablet is simple enough. Our configuration trait can remain empty, although it must exist.

```rust
pub trait Trait: system::Trait {}
```

## Dispatchable Calls

A Dispatch call is a function that can be called by blockchain users as an external part. "Extraneous" is a term for abstract, meaning calls from outside the chain. Most of the time, they are transactions. At present, it is all right to regard them as transactions. Dispatchable call is defined in [decl_moudle!](https://substrate.dev/rustdocs/v2.0.0/frame_support/macro.decl_module.html).

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

As you can see, our hello-substrate pallet has a dispatchable call and a parameter called the origin. This call returns [DispatchResult](https://substrate.dev/rustdocs/v2.0.0/frame_support/dispatch/type.DispatchResult.html), which can be Ok (()) to indicate that the call was successful, or Err to indicate that the call failed. This is demonstrated in most of the drawers.

## Weight Annotations

Before looking at the functions of hello-substrate, we see a line # [weight=10_000], which gives a default weight to calls. The final weight will affect the cost the user needs to pay for calling the function. Weight is a very interesting aspect of using abstract development. However, they will also discuss it later in the [weight](https://substrate.dev/recipes/weights.html) section. Now many recipe palettes will simply use the default weight.

## Inside a Dispatchable Call

Let's take a closer look at our dispatchable call

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

This function mainly does three things. First, it uses [ensure_signed](https://substrate.dev/rustdocs/v2.0.0/frame_system/fn.ensure_signed.html) function to ensures that the caller of the function is an ordinary user with a private key. This function also returns who the caller is. We put the caller's identification in the callet variable.

Secondly, it outputs a message and records the caller. Note that we do not use the normal println! of rust, but a special [print](https://substrate.dev/rustdocs/v2.0.0/sp_runtime/fn.print.html) function and [debug:: info!](https://substrate.dev/rustdocs/v2.0.0/frame_support/debug/macro.info.html), the reason will be explained in the next section.

Finally, the call returns OK(()) indicates that the call is successful. At first glance, it seems that there is no way to make the call fail, but this is not the case. If the call does not come from a signed origin, the start of the ensure_signed function can return an error. This is the first time we have seen an important example of "verification before writing". In the development of subordinates, it is important to always ensure that the prerequisites are met and an error is returned at the beginning. After these checks are completed, you can start the calculation of the function.

## Printing From the Runtime 

Printing to the terminal is usually very simple from the trust program println!, but the subtract runtime is compiled into WebAssembly and regular binary files, and cannot access the standard library of trust, which means that we cannot use regular println! We encourage you to modify the code, try to use println, and confirm that it will not compile. However, printing messages from runtime is useful for recording information and debugging.

![test](../pic/substrate-architecture.png)

At the top of our palette, we imported [sp_runtime print](https://substrate.dev/rustdocs/v2.0.0/sp_runtime/fn.print.html) function. This special function allows the runtime to send a print message outside the node. This message has not been compiled to wasm, and it can access the standard library and execute regular IO. This function can only print items that implement [Printable triat](https://substrate.dev/rustdocs/v2.0.0/sp_runtime/traits/trait.Printable.html). Fortunately, all basic types have implemented this trait, and you can also wie your own data type to implement this trait.

Print function not: To actually print messages, we need to use the flag -lruntime=debug when running the kitchen node. Therefore, the command changes to

./target/release/kitchen-node --dev -lruntime=debug

The next line demonstrates how to use debug:: info! displays the log on the screen and checks the content of the variable. The syntax is very similar to the regular println rust.

Runtime logger note "When we run in native mode, the debug:: info! Information is printed. If we run in wasm, we need an additional step to initialize the [RuntimeLogger](https://substrate.dev/rustdocs/v2.0.0/frame_support/debug/struct.RuntimeLogger.html).