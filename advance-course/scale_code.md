# SCALE 编解码

SCALE(Simplae Concatenated Aggregate Little-Endian) 编解码器是一个轻量级、高效的二进制序列化与反序列化编解码器。

它是专为资源受限的执行环境所设计的，比如Substrate runtime, 可对数据进行高效、无需拷贝的编码和解码、它无法进行自我描述，并假定
解码环境拥有编码数据的所有类型信息。

## Substrate 中的SCALE 

substrate 使用parity-scale-codec 它是SCALE编解码器的Rust实现。采用这个库和SCALE编解码器会使Substrate和区块链系统会更有优势，因为：
- 相对于serde这样的通用序列化框架来说，它是轻量级的，因为serde添加了大量的模板，会使二进制文件过大。
- 他不使用Rust std, 因此可以把文件编译为Wasm格式，用在Substrate runtime之中。
- 他对Rust中为新数据类型派生编解码逻辑的支持极为友好： #[derive(Encode, Decode)]

相对于重用现有的Rust编解码器库而言，定义出Substrate上使用的编码方案非常重要，因为这个编解码需要在其他支持
互操作性的平台和语言上重新实现。 

## 内容
- 序列化与反序列
- SCALE 编解码原理
- 实现

## 数据序列

基本整数类型是由定宽little-endian(LE)编码格式进行编码的。 

数据对象转换成二进制码，高效地进行存储和传输；反之，以相同规则将二进制码解码，可以获得原始数据:
- Bitcoin specific serialization format
- RLP (Recursive length prefix) 以太坊
- SCALE substrate

## SCALE Codec 介绍

简单拼接聚合的小端数据格式(Simple Concatenated Aggregate LittleEndian),

SCALE编码格式主要用在: substrate的runtime 或者嵌入式，链上交易的存储，和交易传输编码

- 轻量、高效的二进制格式
- 适用于Blockchain runtime， 低内存的资源有限环境
- 链上数据和交易传输的编码格式
- 不包含类型信息，解码调用方必须有类型信息。
- 新类型， #[derive(Encode, Decode)]
- 不同的类型对应的编码规则是不同的

## SCALE Codec 原理

### 整数压缩编码

紧凑型或通用整数编码对于编码大整数（最大2^256)来说已经足够的了而且，对大多数值的编码会比定宽版本更有效率。 

固定宽度整数：例: u8, i8, u32, i32, ...
- i8: 69, binary: 0100, 0101, hex: 0x45 // 小端编码
- u16: 42, binary: 0000 0000, 0010 1010, hex: 0x2a00 // 小端存储
- u32: 16777215, binary: 0000 0000, 1111 1111, 1111 1111, 1111 1111, hex: 0xff ff ff 00 

整数的压缩(compact)编码， 编码大整数更高效，最大值2^236,
- 整数类型前标记
    - 整数作为参数时: #[compact]
    - 结构体：#[codec(compact)]

- 最低位的两个bit位表示模式
    - 0b00, 单字节模式， 高6位是值的LE编码（0~63）
    - 0b01, 两字节模式，高6位和下一个字节是值的LE编码（64 ~ （2^14 - 1)
    - 0b10, 四字节，高6位和下3个字节是值的LE编码（(2^14 - 1) ~ (2^30 - 1))
    - 0b11, 大整数模式，高6位表示用来编码值的字节数减去4， 之后的字节是值的编码((2^30 - 1) ~(2^236 - 1))

例子： hex是小端LE
- unsigned integer 0, binary origin: 0000 0000, 0b00 mode, binary add mode : 0000 0000, hex: 0x00
- unsigned integer 1, binary origin: 0000 0001, 0b00 mode , binary add mode : 0000 0100, hex: 0x04
- unsigned integer 42, binary origin: 0010 1010, 0b00mode, binary add mode: 1010 1000, hex: 0xa8
- unsigned integer 69, binary origin: 0100 0101, 0b01 mode, binary add mode: 0000 0001, 0001 0101 hex: 0x1501

### 布尔值

布尔值，单字节的最小位表示，
- false, binary: 0000 0000, hex: 0x00
- true, binary: 0000 0001, hex: 0x01

### Option<T>类型

- 如果有值， 将保存的值编码后拼接， 如Option<i8>
    - None, binary: 0000 0000, hex: 0x00
    - Some(69), binary: 0000 0001, 0100 0101, hex: 0x0145
- 特例：
    - None, hex: 0x00
    - Some(true), hex: 0x01
    - Some(false), hex: 0x02

### Result<T, E> 类型

- 0x00 表示Ok(v), 后面紧跟值v的编码
- 0x01 表示Err(e), 后面紧跟错误信息e的编码

例如 type MyResult = std::result::Result<u8, bool>;
- Ok(42), hex: 0x002a
- Err(false), hex: 0x0100

## Vectors(list, series, sets)

以集合内元素数量的compact编码开始，紧跟各个元素值的编码，按顺序拼接,例如：

origin: u16 整数的集合， [4, 8, 15, 16, 23, 42] 共6个元素
binary: 
0001 1000 (6 in compact),
0000 0000, 0000 0100 (4), 0000 0000, 0000 1000 (8),
0000 0000, 0000 1111 (15), 0000 0000, 0001 0000 (16),
0000 0000, 0001 0111 (23), 0000 0000, 0010 1010 (42),
hex: 0x 18 0400 0800 0f00 1000 1700 2a00

## 字符串string 

- 以Vec<u8>的形式进行表示和编码
- u8数值来源于字符的UTF8编码

### 元组

一个固定大小的数值序列，每个数值都可以是不同类型，但这些类型是预先确定且不可更改的。编码时就是简单把所有的值编码连接起来。


各个元素的编码直接拼接

- OriginL (3, false), binary: 0000 1100, 0000 0000, hex: 0x0c00

## 结构体

- 属性名不会被编码到结果中
- 和元组类似，通常是各个属性值的编码直接拼接

对于结构体而言，虽然值被命名了，但编码并不适用该名字（名字是被忽略的- 仅值的顺序重要）所有容器都是连续
存储元素的，元素的存储顺序取决于容器，而不是一成不变的，因此不能依赖于解码。 

这就意味着，如果将某个字节数组解码成一个指定的结构体（结构体本身是排好序的），然后再重新编码，可能会导致
新编码的数组与原有的字节数组不一样。 


```rust
struct MyStruct {
    #[codec(compact)]
    a: u32, 
    b: bool,
}

let my_struct = MyStruct {
    a: 42, 
    b: true,
};

binary: 0010 1010, 0000 0001
binary add mode : 1010 1000, 0000 0001
hex: 0xa8 01
```
## 枚举 Enum 

固定数量的变量，每一个变量都是互斥的，并可能还会包含另一个或者另一系列的值。

编码后的第一个字节表示值的所在变量的索引，后续的字节是该变量所包含的实际值的编码结果，因此最多支持256个变量。

第一个字节用来标示变体的位置，即最多支持256个变体，其后的内容用来编码变体里可能包含的值。

- #[codec(index = "1")]， 指定某个变体的index编码

```rust
enum IntOrBool {
    Int(u8),
    Bool(bool),
}
```

- Int(42), hex: 0x002a
- Bool(true), hex: 0x0101

## SCALE Codec实现

- Rust: paritytech/parity-scale-codec
- python: polkascan/py-scale-codec
- golang: itering/scale.go
- c++: soramitsu/scale
- javascript: polkadot-js/api
- assemblyScript: LimeChaim/as-scale-codec
- Haskell: airalab/hs-web3
- java: emeraldpay/polkaj
- ruby: itering/scale.rb

## SCALE Codec文档

https://substrate.dev/docs/en/knowledgebase/advanced/codec
