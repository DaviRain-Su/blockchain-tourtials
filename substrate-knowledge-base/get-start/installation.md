# Installation

This page will guide you through the steps needed to prepare a computer for Substrate development. Since Substrate is built with [the Rust programming language](https://www.rust-lang.org/), the first thing you will need to do is prepare the computer for Rust development - these steps will vary based on the computer's operating system. Once Rust is configured, you will use its toolchains to interact with Rust projects; the commands for Rust's toolchains will be the same for all supported, Unix-based operating systems.

本页将指导你完成为Substrate开发准备计算机所需的步骤。由于Substrate是用Rust编程语言构建的，所以你需要做的第一件事是为Rust开发准备计算机--这些步骤将根据计算机的操作系统而变化。一旦配置了Rust，您将使用它的工具链与Rust项目进行交互；Rust工具链的命令对于所有支持的、基于Unix的操作系统都是一样的。

## Unix-Based Operating Systems

Substrate development is easiest on Unix-based operating systems like macOS or Linux. The examples in the Substrate [Tutorials](https://substrate.dev/tutorials) and [Recipes](https://substrate.dev/recipes/) use Unix-style terminals to demonstrate how to interact with Substrate from the command line.

Substrate的开发在基于Unix的操作系统上是最简单的，比如macOS或Linux。Substrate Tutorials and Recipes中的例子使用Unix风格的终端来演示如何从命令行与Substrate交互。

### macos

Open the Terminal application and execute the following commands:

打开终端应用程序并执行以下命令。

```shell
# Install Homebrew if necessary https://brew.sh/
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"

# Make sure Homebrew is up-to-date, install openssl and cmake
brew update
brew install openssl cmake
```

### Ubuntu/Debian

Use a terminal shell to execute the following commands:

使用终端shell执行以下命令。

```shell
sudo apt update
# May prompt for location information
sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev curl
```

### Arch Linux

Run these commands from a terminal:

从终端运行这些命令

```shell
pacman -Syu --needed --noconfirm cmake gcc openssl-1.0 pkgconf git clang
export OPENSSL_LIB_DIR="/usr/lib/openssl-1.0"
export OPENSSL_INCLUDE_DIR="/usr/include/openssl-1.0"
```

## Windows

Please refer to the separate [guide for Windows users](https://substrate.dev/docs/en/knowledgebase/getting-started/windows-users).

请参考Windows用户的单独指南。

## Rust Developer Environment

This guide uses [`rustup`](https://rustup.rs/) to help manage the Rust toolchain. First install and configure `rustup`:

本指南使用rustup来帮助管理Rust工具链。首先安装并配置rustup。

```bash
# Install
curl https://sh.rustup.rs -sSf | sh
# Configure
source ~/.cargo/env

Copy
```

Configure the Rust toolchain to default to the latest stable version:

将Rust工具链配置为默认为最新的稳定版本。

```bash
rustup default stable

Copy
```

### WebAssembly Compilation

Substrate uses [WebAssembly](https://webassembly.org/) (Wasm) to produce portable blockchain runtimes. You will need to configure your Rust compiler to use [`nightly` builds](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) to allow you to compile Substrate runtime code to the Wasm target.

Substrate使用WebAssembly（Wasm）来制作可移植的区块链运行时。你需要配置你的Rust编译器，以使用夜间构建，允许你将Substrate运行时代码编译到Wasm目标。

#### Rust Nightly Toolchain

Developers building with Substrate should use a specific Rust nightly version that is known to be compatible with the version of Substrate they are using; this version will vary from project to project and different projects may use different mechanisms to communicate this version to developers. For instance, the Polkadot client specifies this information in its [release notes](https://github.com/paritytech/polkadot/releases). The Substrate Node Template uses an [init script](https://github.com/substrate-developer-hub/substrate-node-template/blob/master/scripts/init.sh) and [Makefile](https://github.com/substrate-developer-hub/substrate-node-template/blob/master/Makefile) to specify the Rust nightly version and encapsulate the following steps. Use Rustup to install the correct nightly:

使用Substrate进行构建的开发者应该使用已知与他们所使用的Substrate版本兼容的特定Rust夜间版本，这个版本会因项目而异，不同的项目可能会使用不同的机制来向开发者传达这个版本。例如，Polkadot客户端在其发布说明中指定了这个信息。Substrate Node模板使用init脚本和Makefile来指定Rust夜间版本，并封装以下步骤。使用Rustup安装正确的年夜型。

```bash
rustup install nightly-<yyyy-MM-dd>

Copy
```

#### Wasm Toolchain

Now, configure the nightly version to work with the Wasm compilation target:

现在，配置夜间版本与Wasm编译目标一起工作。

```bash
rustup target add wasm32-unknown-unknown --toolchain nightly-<yyyy-MM-dd>

Copy
```

#### Specifying Nightly Version

Use the `WASM_BUILD_TOOLCHAIN` environment variable to specify the Rust nightly version a Substrate project should use for Wasm compilation:

使用WASM_BUILD_TOOLCHAIN环境变量来指定Substrate项目在Wasm编译时应该使用的Rust夜间版本。

```bash
WASM_BUILD_TOOLCHAIN=nightly-<yyyy-MM-dd> cargo build --release

Copy
```

Note that this only builds *the runtime* with the specified nightly. The rest of project will be compiled with the default toolchain, i.e. the latest installed stable toolchain.

请注意，这只是用指定的 nightly 构建*运行时。项目的其余部分将使用默认的工具链，即最新安装的稳定工具链进行编译

#### Latest Nightly for Substrate `master`

Developers that are building Substrate *itself* should always use the latest bug-free versions of Rust stable and nightly. This is because the Substrate codebase follows the tip of Rust nightly, which means that changes in Substrate often depend on upstream changes in the Rust nightly compiler. To ensure your Rust compiler is always up to date, you should run:

开发者在构建Substrate时，应该始终使用最新的Rust稳定版和nightly的无bug版本。这是因为Substrate代码库跟随Rust nightly的提示，这意味着Substrate的变化经常依赖于Rust nightly编译器的上游变化。为了确保你的Rust编译器始终是最新的，你应该运行。

```bash
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

Copy
```

**It may be necessary to occasionally rerun `rustup update`** if a change in the upstream Substrate codebase depends on a new feature of the Rust compiler.

如果上游Substrate代码库的变化依赖于Rust编译器的新特性，可能需要偶尔重新运行rustup更新。

#### Downgrading Rust Nightly

If your computer is configured to use the latest Rust nightly and you would like to downgrade to a specific nightly version, follow these steps:

如果您的计算机配置为使用最新的Rust nightly，并且您想降级到特定的nightly版本，请按照以下步骤进行。

```sh
rustup uninstall nightly
rustup install nightly-<yyyy-MM-dd>
rustup target add wasm32-unknown-unknown --toolchain nightly-<yyyy-MM-dd>

Copy
```

## Test Your Set-Up

The best way to ensure that you have successfully prepared a computer for Substrate development is to follow the steps in our first tutorial, [Create Your First Substrate Chain](https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/).

确保你已经成功地为Substrate开发准备了一台计算机的最好方法是按照我们的第一个教程 "创建你的第一条Substrate链 "中的步骤进行。