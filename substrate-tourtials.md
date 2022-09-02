# 知乎金晓 Substrate 文章

https://zhuanlan.zhihu.com/p/94624571

# 区块链学习路线整理

https://zhuanlan.zhihu.com/p/40100932



# 环境配置

```
cargo run -- <参数>  # 注意这里有两个横杆 --
# 等于
cargo build && ./target/debug/<项目执行文件> <参数>
# 而 release编译
cargo run --release -- <参数>
# 等于
cargo build --release && ./target/release/<执行文件> <参数>
```

### 注意的点：

1.wasm是否以release编译

查看 `target/<debug/release>/wbuild/node-runtime/node_runtime.compact.wasm`的大小，若其大小是1.3M左右，可确定是以release编译，若是8M以上，则一定是以debug编译，此时建议设置好相应环境变量重新编译。

2.编译后的执行文件

编译后产生的执行文件有4个

- substrate # 即 substrate的node项目执行文件，源码位于`bin/node`，研究substrate的基础入口，cargo run 执行的文件即为该文件
- node-rpc-client # 与substrate node进行交互的rpc执行文件，源码位于`bin/node/rpc-client`下，注意这个和node交互，不是与node-template交互
- node-template # 精简版的node，源码位于`bin/node-template`，与node相比去除了大部分runtime模块，可最为最精简链运行
- subkey # 用于生成一些公私钥的工具，源码位于`bin/substrate`

对于substrate的研究，只需要关心node项目即可，不擅长js的，可用辅助使用node-rpc-client用来发交易。





# Substrate help

substrate 2.0.0-3c531e291-x86_64-macos



Parity Technologies <admin@parity.io>

Generic Substrate node implementation in Rust.



USAGE:

  substrate [FLAGS] [OPTIONS]

  substrate <SUBCOMMAND>



FLAGS:

​    --alice               

​      Shortcut for `--name Alice --validator` with session keys for `Alice` added to keystore



​    --bob                

​      Shortcut for `--name Bob --validator` with session keys for `Bob` added to keystore



​    --charlie              

​      Shortcut for `--name Charlie --validator` with session keys for `Charlie` added to keystore



​    --dave               

​      Shortcut for `--name Dave --validator` with session keys for `Dave` added to keystore



​    --dev                

​      Specify the development chain



​    --disable-log-reloading       

​      Disable feature to dynamically update and reload the log filter.

​       

​      By default this feature is enabled, however it leads to a small performance decrease. The

​      `system_addLogFilter` and `system_resetLogFilter` RPCs will have no effect with this option set.

​    --discover-local          

​      Enable peer discovery on local networks.

​       

​      By default this option is `true` for `--dev` or when the chain type is `Local`/`Development` and false

​      otherwise.

​    --eve                

​      Shortcut for `--name Eve --validator` with session keys for `Eve` added to keystore



​    --ferdie              

​      Shortcut for `--name Ferdie --validator` with session keys for `Ferdie` added to keystore



​    --force-authoring          

​      Enable authoring even when offline



  -h, --help               

​      Prints help information



​    --kademlia-disjoint-query-paths   

​      Require iterative Kademlia DHT queries to use disjoint paths for increased resiliency in the presence of

​      potentially adversarial nodes.

​       

​      See the S/Kademlia paper for more information on the high level design as well as its security improvements.

​    --light               

​      Experimental: Run in light client mode



​    --no-grandpa            

​      Disable GRANDPA voter when running in validator mode, otherwise disable the GRANDPA observer



​    --no-mdns              

​      Disable mDNS discovery.

​       

​      By default, the network will use mDNS to discover other nodes on the local network. This disables it.

​      Automatically implied when using --dev.

​    --no-private-ipv4          

​      Forbid connecting to private IPv4 addresses (as specified in

​      [RFC1918](https://tools.ietf.org/html/rfc1918)), unless the address was passed with `--reserved-nodes` or

​      `--bootnodes`

​    --no-prometheus           

​      Do not expose a Prometheus metric endpoint.

​       

​      Prometheus metric endpoint is enabled by default.

​    --no-telemetry           

​      Disable connecting to the Substrate telemetry server.

​       

​      Telemetry is on by default on global chains.

​    --one                

​      Shortcut for `--name One --validator` with session keys for `One` added to keystore



​    --password-interactive       

​      Use interactive shell for entering the password used by the keystore



​    --prometheus-external        

​      Listen to all Prometheus data source interfaces.

​       

​      Default is local.

​    --reserved-only           

​      Whether to only allow connections to/from reserved nodes.

​       

​      If you are a validator your node might still connect to other validator nodes regardless of whether they are

​      defined as reserved nodes.

​    --rpc-external           

​      Listen to all RPC interfaces.

​       

​      Default is local. Note: not all RPC methods are safe to be exposed publicly. Use an RPC proxy server to

​      filter out dangerous methods. More details: <https://github.com/paritytech/substrate/wiki/Public-RPC>. Use

​      `--unsafe-rpc-external` to suppress the warning if you understand the risks.

​    --tmp                

​      Run a temporary node.

​       

​      A temporary directory will be created to store the configuration and will be deleted at the end of the

​      process.

​       

​      Note: the directory is random per process execution. This directory is used as base path which includes:

​      database, node key and keystore.

​    --two                

​      Shortcut for `--name Two --validator` with session keys for `Two` added to keystore



​    --unsafe-pruning          

​      Force start with unsafe pruning settings.

​       

​      When running as a validator it is highly recommended to disable state pruning (i.e. 'archive') which is the

​      default. The node will refuse to start as a validator if pruning is enabled unless this option is set.

​    --unsafe-rpc-external        

​      Listen to all RPC interfaces.

​       

​      Same as `--rpc-external`.

​    --unsafe-ws-external        

​      Listen to all Websocket interfaces.

​       

​      Same as `--ws-external` but doesn't warn you about it.

​    --validator             

​      Enable validator mode.

​       

​      The node will be started with the authority role and actively participate in any consensus task that it can

​      (e.g. depending on availability of local keys).

  -V, --version              

​      Prints version information



​    --ws-external            

​      Listen to all Websocket interfaces.

​       

​      Default is local. Note: not all RPC methods are safe to be exposed publicly. Use an RPC proxy server to

​      filter out dangerous methods. More details: <https://github.com/paritytech/substrate/wiki/Public-RPC>. Use

​      `--unsafe-ws-external` to suppress the warning if you understand the risks.



OPTIONS:

  -d, --base-path <PATH>                    

​      Specify custom base path



​    --bootnodes <ADDR>...                   

​      Specify a list of bootnodes



​    --chain <CHAIN_SPEC>                   

​      Specify the chain specification.

​       

​      It can be one of the predefined ones (dev, local, or staging) or it can be a path to a file with the

​      chainspec (such as one exported by the `build-spec` subcommand).

​    --database <DB>                      

​      Select database backend to use



​    --db-cache <MiB>                     

​      Limit the memory the database cache can use



​    --offchain-worker <ENABLED>

​      Should execute offchain workers on every block.

​       

​      By default it's only enabled for nodes that are authoring new blocks. [default: WhenValidating] [possible

​      values: Always, Never, WhenValidating]

​    --execution <STRATEGY>

​      The execution strategy that should be used by all execution contexts [possible values: Native, Wasm, Both,

​      NativeElseWasm]

​    --execution-block-construction <STRATEGY>

​      The means of execution used when calling into the runtime while constructing blocks [possible values:

​      Native, Wasm, Both, NativeElseWasm]

​    --execution-import-block <STRATEGY>

​      The means of execution used when calling into the runtime for general block import (including locally

​      authored blocks) [possible values: Native, Wasm, Both, NativeElseWasm]

​    --execution-offchain-worker <STRATEGY>

​      The means of execution used when calling into the runtime while using an off-chain worker [possible values:

​      Native, Wasm, Both, NativeElseWasm]

​    --execution-other <STRATEGY>

​      The means of execution used when calling into the runtime while not syncing, importing or constructing

​      blocks [possible values: Native, Wasm, Both, NativeElseWasm]

​    --execution-syncing <STRATEGY>

​      The means of execution used when calling into the runtime for importing blocks as part of an initial sync

​      [possible values: Native, Wasm, Both, NativeElseWasm]

​    --in-peers <COUNT>

​      Specify the maximum number of incoming connections we're accepting [default: 25]



​    --enable-offchain-indexing <ENABLE_OFFCHAIN_INDEXING>

​      Enable Offchain Indexing API, which allows block import to write to Offchain DB.

​       

​      Enables a runtime to write directly to a offchain workers DB during block import.

​    --ipc-path <PATH>                     

​      Specify IPC RPC server path



​    --keystore-path <PATH>                  

​      Specify custom keystore path



​    --keystore-uri <keystore-uri>               

​      Specify custom URIs to connect to for keystore-services



​    --listen-addr <LISTEN_ADDR>...              

​      Listen on this multiaddress



  -l, --log <LOG_PATTERN>...

​      Sets a custom logging filter. Syntax is <target>=<level>, e.g. -lsync=debug.

​       

​      Log levels (least to most verbose) are error, warn, info, debug, and trace. By default, all targets log

​      `info`. The global log level can be set with -l<level>.

​    --max-parallel-downloads <COUNT>

​      Maximum number of peers from which to ask for the same blocks in parallel.

​       

​      This allows downloading announced blocks from multiple peers. Decrease to save traffic and risk increased

​      latency. [default: 5]

​    --max-runtime-instances <max-runtime-instances>      

​      The size of the instances cache for each runtime.

​       

​      The default value is 8 and the values higher than 256 are ignored.

​    --name <NAME>                       

​      The human-readable name for this node.

​       

​      The node name will be reported to the telemetry server, if enabled.

​    --node-key <KEY>                     

​      The secret key to use for libp2p networking.

​       

​      The value is a string that is parsed according to the choice of `--node-key-type` as follows:

​       

​      `ed25519`: The value is parsed as a hex-encoded Ed25519 32 byte secret key, i.e. 64 hex characters.

​       

​      The value of this option takes precedence over `--node-key-file`.

​       

​      WARNING: Secrets provided as command-line arguments are easily exposed. Use of this option should be limited

​      to development and testing. To use an externally managed secret key, use `--node-key-file` instead.

​    --node-key-file <FILE>

​      The file from which to read the node's secret key to use for libp2p networking.

​       

​      The contents of the file are parsed according to the choice of `--node-key-type` as follows:

​       

​      `ed25519`: The file must contain an unencoded 32 byte or hex encoded Ed25519 secret key.

​       

​      If the file does not exist, it is created with a newly generated secret key of the chosen type.

​    --node-key-type <TYPE>

​      The type of secret key to use for libp2p networking.

​       

​      The secret key of the node is obtained as follows:

​       

​      \* If the `--node-key` option is given, the value is parsed as a secret key according to the type. See the

​      documentation for `--node-key`.

​       

​      \* If the `--node-key-file` option is given, the secret key is read from the specified file. See the

​      documentation for `--node-key-file`.

​       

​      \* Otherwise, the secret key is read from a file with a predetermined, type-specific name from the chain-

​      specific network config directory inside the base directory specified by `--base-dir`. If this file

​      does not exist, it is created with a newly generated secret key of the chosen type.

​       

​      The node's secret key determines the corresponding public key and hence the node's peer ID in the context of

​      libp2p. [default: Ed25519] [possible values: Ed25519]

​    --out-peers <COUNT>

​      Specify the number of outgoing connections we're trying to maintain [default: 25]



​    --password <password>                   

​      Password used by the keystore



​    --password-filename <PATH>                

​      File that contains the password used by the keystore



​    --pool-kbytes <COUNT>

​      Maximum number of kilobytes of all transactions stored in the pool [default: 20480]



​    --pool-limit <COUNT>

​      Maximum number of transactions in the transaction pool [default: 8192]



​    --port <PORT>                       

​      Specify p2p protocol TCP port



​    --prometheus-port <PORT>                 

​      Specify Prometheus data source server TCP Port



​    --pruning <PRUNING_MODE>

​      Specify the state pruning mode, a number of blocks to keep or 'archive'.

​       

​      Default is to keep all block states if the node is running as a validator (i.e. 'archive'), otherwise state

​      is only kept for the last 256 blocks.

​    --public-addr <PUBLIC_ADDR>...

​      The public address that other nodes will use to connect to it. This can be used if there's a proxy in front

​      of this node

​    --reserved-nodes <ADDR>...                

​      Specify a list of reserved node addresses



​    --rpc-cors <ORIGINS>

​      Specify browser Origins allowed to access the HTTP & WS RPC servers.

​       

​      A comma-separated list of origins (protocol://domain or special `null` value). Value of `all` will disable

​      origin validation. Default is to allow localhost and <https://polkadot.js.org> origins. When running in

​      --dev mode the default is to allow all origins.

​    --rpc-methods <METHOD SET>

​      RPC methods to expose.

​       

​      \- `Unsafe`: Exposes every RPC method.

​      \- `Safe`: Exposes only a safe subset of RPC methods, denying unsafe RPC methods.

​      \- `Auto`: Acts as `Safe` if RPC is served externally, e.g. when `--{rpc,ws}-external` is passed,

​       otherwise acts as `Unsafe`. [default: Auto] [possible values: Auto, Safe, Unsafe]

​    --rpc-port <PORT>                     

​      Specify HTTP RPC server TCP port



​    --sentry <sentry>...                   

​      Enable sentry mode.

​       

​      The node will be started with the authority role and participate in consensus tasks as an "observer", it

​      will never actively participate regardless of whether it could (e.g. keys are available locally). This mode

​      is useful as a secure proxy for validators (which would run detached from the network), since we want this

​      node to participate in the full consensus protocols in order to have all needed consensus data available to

​      relay to private nodes.

​    --sentry-nodes <ADDR>...                 

​      Specify a list of sentry node public addresses.

​       

​      Can't be used with --public-addr as the sentry node would take precedence over the public address specified

​      there.

​    --state-cache-size <Bytes>                

​      Specify the state cache size [default: 67108864]



​    --telemetry-url <URL VERBOSITY>...            

​      The URL of the telemetry server to connect to.

​       

​      This flag can be passed multiple times as a means to specify multiple telemetry endpoints. Verbosity levels

​      range from 0-9, with 0 denoting the least verbosity. Expected format is 'URL VERBOSITY', e.g. `--telemetry-

​      url 'wss://foo/bar 0'`.

​    --tracing-receiver <RECEIVER>

​      Receiver to process tracing messages [default: Log] [possible values: Log, Telemetry]



​    --tracing-targets <TARGETS>

​      Sets a custom profiling filter. Syntax is the same as for logging: <target>=<level>



​    --wasm-execution <METHOD>

​      Method for executing Wasm runtime code [default: Interpreted] [possible values: Interpreted, Compiled]



​    --wasm-runtime-overrides <PATH>              

​      Specify the path where local WASM runtimes are stored.

​       

​      These runtimes will override on-chain runtimes when the version matches.

​    --ws-max-connections <COUNT>               

​      Maximum number of WS RPC server connections



​    --ws-port <PORT>                     

​      Specify WebSockets RPC server TCP port





SUBCOMMANDS:

  benchmark    Benchmark runtime pallets.

  build-spec    Build a chain specification

  check-block   Validate blocks

  export-blocks  Export blocks

  export-state   Export the state of a given block into a chain spec

  help       Prints this message or the help of the given subcommand(s)

  import-blocks  Import blocks

  inspect     Decode given block or extrinsic using current native runtime.

  key       Key management cli utilities

  purge-chain   Remove the whole chain

  revert      Revert the chain to a previous state

  sign       Sign a message, with a given (secret) key

  vanity      Generate a seed that provides a vanity address

  verify      Verify a signature for a message, provided on STDIN, with a given (public or secret) key