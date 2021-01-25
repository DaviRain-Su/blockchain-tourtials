# 链下工作机(Off-chian Worker)

## Substrate 中的密码学

- 哈希键生成方法
- 钥匙对生成及签名法

## 哈希键值生成方法

### 单向散列函数(on-way hash function) Hash函数的特点

- 输入可以任意长度，输出是固定长度
  - 不论输入的信息的长度有多长，只要输入hash函数，输出的信息都是固定长度的比特值。比如sha256,输入任何的值出来的都是256比特的0或1
- 计算hash值的速度比较快
  - 单向哈希的计算很快，才能保证加密或者验证的速度
- 防碰撞特性(Collision resistance)
  - x != y， H(x) = H(y) 输入空间远远大于输出空间，比如256位的哈希值指的就是输出空间是2^256，输入是无限可能的，输出是固定长度。
  - 没有一个好的方法能找到一个x使得得到的H(x)等于H(y)
  - 只能通过暴力破解的方式去找到这个值，遍历所有的输入的可能去找到这个值，这叫做brute-force暴力破解
  - 输入的一点点的改变都会造成输出的hash值不一样
  - collision resistance 目前还没有数学证明这个碰撞不会发生，MD5就是一个例子，之前是安全地，后来找到了破解的方法

- 隐藏性(Hiding)或者叫做单向性(one-way)
  - 哈希函数的计算过程是单向不可逆的，x推出H(X)，但是反推没有办法，也就是说，哈希值没有泄露输入的x的信息，也就说x的信息被隐藏起来了。这就是隐藏性
  - 输入空间要足够的大，取值是均匀的，这样就很难暴力破解
  - 例子
    - 一个有影响力的股票评价师，今天预测一下明天的股价是不是增长，那么，他如果公开表明币价，可能会影响币价。
    - 如何表明他确实很准确呢？让他把股评嘻嘻写到纸上，或者存到电脑里，但是要求在第二天开盘后不能修改内容，这样就不用担心预测影响股价了，所以，保证他没有篡改自己已经写好的内容。
    - 可以使用hash算法，预测的结果是x，对x求hash函数，公布hash值，第二天再将x公布出来，如果你修改了昨天的数据，hash值就变了，所有人都可以用hash算一下这个x和昨天公布的hash值进行对比
    - 一般在求hash值的时候还会加上一个nonce值去求hash值
- 谜题友好型(Puzzle friendly)
  - 无法从输入数据，判断出到底输出的是什么
  - 知道输入的信息，无法一眼看出来输出的hash值是什么，谜题友好性，就是这样的，你无法通过控制输入x来获得你想要的输出值H(x)
  - 综合隐藏性和谜题友好的两个特点，知道输入信息也不知道hash值是什么，可以很快算出来，但是无法预先判断，知道哈希值也不能知道输入值是什么，反向计算是非常困难的，只能暴力破解
  - 所以要想输出的值落在某一个范围里，比如小于某个数值，计算机只能一个一个的去试。

###  map 键生产的方法：

- Identity: 对参数不作加密处理，直接作为键值来用，通常是用在hash的键参数上，而不是用户控制的值上的
  - `identity`: This is not a hasher at all, and just uses the key material directly. Since it does no hashing or appending, it's the fastest possible hasher, however, it's also the least secure. It can be used only if you know that the key will be cryptographically/securely randomly distributed over the binary encoding space. In most cases this will not be true. One case where it is true, however, if where the key is itself the result of a cryptographic hash of some existent data.
- Twox_64_concat: 优点是非常的快，以及支持map可遍历它的所有键，缺点是密码学上不是绝对安全
  - `twox_64_concat`: This is an insecure hasher and can only be used safely if you know that the preimages cannot be chosen at will by untrusted users. It is memory-efficient, extremely performant and supports iteration over its keys and values. You can safely use this is the key is:
    - A (slowly) incrementing index.
    - Known to be the result of a cryptographic hash (though `identity` is a better choice here).
    - Known to be the public key of a cryptographic key pair in existence.
- blake2_128_concat: 优点是密码学上相对安全，也支持该map可遍历它的所有键，缺点是需要一定计算量，相比twox_64_concat 要慢
  - `blake2_128_concat`: The default, safe choice. Use if you are unsure or don't care. It is secure against user-tainted keys, fairly fast and memory-efficient and supports iteration over its keys and values. This must be used if the keys of your map can be selected *en masse* by untrusted users.

如果实在不知道选择那个，那就选择blake2_128_concat

参考链接：

https://substrate.dev/docs/en/knowledgebase/advanced/cryptography

https://wiki.polkadot.network/docs/en/learn-cryptography

https://substrate.dev/rustdocs/v2.0.0/frame_support/macro.decl_storage.html





## 钥匙对生成及签名法

因为实现的是钥匙对，所以在Substrate 中，所有的钥匙对的实例都得实现 [Pair triat](https://substrate.dev/rustdocs/v2.0.0/sp_core/crypto/trait.Pair.html) 

```rust
// Location source code  : substrate/primitives/core/src/crypto.rs

/// Trait suitable for typical cryptographic PKI key pair type.
///
/// For now it just specifies how to create a key from a phrase and derivation path.
#[cfg(feature = "full_crypto")]
pub trait Pair: CryptoType + Sized + Clone + Send + Sync + 'static {
	/// The type which is used to encode a public key.
	type Public: Public + Hash;

	/// The type used to (minimally) encode the data required to securely create
	/// a new key pair.
	type Seed: Default + AsRef<[u8]> + AsMut<[u8]> + Clone;

	/// The type used to represent a signature. Can be created from a key pair and a message
	/// and verified with the message and a public key.
	type Signature: AsRef<[u8]>;

	/// Error returned from the `derive` function.
	type DeriveError;

	/// Generate new secure (random) key pair.
	///
	/// This is only for ephemeral keys really, since you won't have access to the secret key
	/// for storage. If you want a persistent key pair, use `generate_with_phrase` instead.
	#[cfg(feature = "std")]
	fn generate() -> (Self, Self::Seed) {
		let mut seed = Self::Seed::default();
		OsRng.fill_bytes(seed.as_mut());
		(Self::from_seed(&seed), seed)
	}

	/// Generate new secure (random) key pair and provide the recovery phrase.
	///
	/// You can recover the same key later with `from_phrase`.
	///
	/// This is generally slower than `generate()`, so prefer that unless you need to persist
	/// the key from the current session.
	#[cfg(feature = "std")]
	fn generate_with_phrase(password: Option<&str>) -> (Self, String, Self::Seed);

	/// Returns the KeyPair from the English BIP39 seed `phrase`, or `None` if it's invalid.
	#[cfg(feature = "std")]
	fn from_phrase(phrase: &str, password: Option<&str>) -> Result<(Self, Self::Seed), SecretStringError>;

	/// Derive a child key from a series of given junctions.
	fn derive<Iter: Iterator<Item=DeriveJunction>>(&self,
		path: Iter,
		seed: Option<Self::Seed>,
	) -> Result<(Self, Option<Self::Seed>), Self::DeriveError>;

	/// Generate new key pair from the provided `seed`.
	///
	/// @WARNING: THIS WILL ONLY BE SECURE IF THE `seed` IS SECURE. If it can be guessed
	/// by an attacker then they can also derive your key.
	fn from_seed(seed: &Self::Seed) -> Self;

	/// Make a new key pair from secret seed material. The slice must be the correct size or
	/// it will return `None`.
	///
	/// @WARNING: THIS WILL ONLY BE SECURE IF THE `seed` IS SECURE. If it can be guessed
	/// by an attacker then they can also derive your key.
	fn from_seed_slice(seed: &[u8]) -> Result<Self, SecretStringError>;

	/// Sign a message.
	fn sign(&self, message: &[u8]) -> Self::Signature;

	/// Verify a signature on a message. Returns true if the signature is good.
	fn verify<M: AsRef<[u8]>>(sig: &Self::Signature, message: M, pubkey: &Self::Public) -> bool;

	/// Verify a signature on a message. Returns true if the signature is good.
	fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P) -> bool;

	/// Get the public key.
	fn public(&self) -> Self::Public;

	/// Interprets the string `s` in order to generate a key Pair. Returns both the pair and an optional seed, in the
	/// case that the pair can be expressed as a direct derivation from a seed (some cases, such as Sr25519 derivations
	/// with path components, cannot).
	///
	/// This takes a helper function to do the key generation from a phrase, password and
	/// junction iterator.
	///
	/// - If `s` is a possibly `0x` prefixed 64-digit hex string, then it will be interpreted
	/// directly as a `MiniSecretKey` (aka "seed" in `subkey`).
	/// - If `s` is a valid BIP-39 key phrase of 12, 15, 18, 21 or 24 words, then the key will
	/// be derived from it. In this case:
	///   - the phrase may be followed by one or more items delimited by `/` characters.
	///   - the path may be followed by `///`, in which case everything after the `///` is treated
	/// as a password.
	/// - If `s` begins with a `/` character it is prefixed with the Substrate public `DEV_PHRASE` and
	/// interpreted as above.
	///
	/// In this case they are interpreted as HDKD junctions; purely numeric items are interpreted as
	/// integers, non-numeric items as strings. Junctions prefixed with `/` are interpreted as soft
	/// junctions, and with `//` as hard junctions.
	///
	/// There is no correspondence mapping between SURI strings and the keys they represent.
	/// Two different non-identical strings can actually lead to the same secret being derived.
	/// Notably, integer junction indices may be legally prefixed with arbitrary number of zeros.
	/// Similarly an empty password (ending the SURI with `///`) is perfectly valid and will generally
	/// be equivalent to no password at all.
	///
	/// `None` is returned if no matches are found.
	#[cfg(feature = "std")]
	fn from_string_with_seed(s: &str, password_override: Option<&str>)
		-> Result<(Self, Option<Self::Seed>), SecretStringError>
	{
		let re = Regex::new(r"^(?P<phrase>[\d\w ]+)?(?P<path>(//?[^/]+)*)(///(?P<password>.*))?$")
			.expect("constructed from known-good static value; qed");
		let cap = re.captures(s).ok_or(SecretStringError::InvalidFormat)?;

		let re_junction = Regex::new(r"/(/?[^/]+)")
			.expect("constructed from known-good static value; qed");
		let path = re_junction.captures_iter(&cap["path"])
			.map(|f| DeriveJunction::from(&f[1]));

		let phrase = cap.name("phrase").map(|r| r.as_str()).unwrap_or(DEV_PHRASE);
		let password = password_override.or_else(|| cap.name("password").map(|m| m.as_str()));

		let (root, seed) = if phrase.starts_with("0x") {
			hex::decode(&phrase[2..]).ok()
				.and_then(|seed_vec| {
					let mut seed = Self::Seed::default();
					if seed.as_ref().len() == seed_vec.len() {
						seed.as_mut().copy_from_slice(&seed_vec);
						Some((Self::from_seed(&seed), seed))
					} else {
						None
					}
				})
				.ok_or(SecretStringError::InvalidSeed)?
		} else {
			Self::from_phrase(phrase, password)
				.map_err(|_| SecretStringError::InvalidPhrase)?
		};
		root.derive(path, Some(seed)).map_err(|_| SecretStringError::InvalidPath)
	}

	/// Interprets the string `s` in order to generate a key pair.
	///
	/// See [`from_string_with_seed`](Pair::from_string_with_seed) for more extensive documentation.
	#[cfg(feature = "std")]
	fn from_string(s: &str, password_override: Option<&str>) -> Result<Self, SecretStringError> {
		Self::from_string_with_seed(s, password_override).map(|x| x.0)
	}

	/// Return a vec filled with raw data.
	fn to_raw_vec(&self) -> Vec<u8>;
}

```



### Substrate 支持三种钥匙生成及签名算法

- ECDSA: 基于secp256k1曲线的ECDSA签名算法

  - Bitcoin 和Etherenum 都是用这个生成钥匙及签名算法
  - 参考 [secp256k1曲线](https://en.bitcoin.it/wiki/Secp256k1)
  - 参考 [ECDSA 签名算法](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)

- ED25519： 基于25519曲线(Curve 25519)的ECDSA签名算法

  - 参考 [25519曲线](https://en.wikipedia.org/wiki/Curve25519)
  - 参考 [Ed25519](https://en.wikipedia.org/wiki/EdDSA#Ed25519)

- SR25519： 基于受Ristretto压缩算法的25519曲线的Schnorrkel签名算法影响的改进

  - 把25519曲线的一些缺点做了改进，也是Substrate默认开账号时采用的方法

  ```rust
  pub fn development_config() -> Result<ChainSpec, String> {
  	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
  
  	Ok(ChainSpec::from_genesis(
  		// Name
  		"Development",
  		// ID
  		"dev",
  		ChainType::Development,
  		move || testnet_genesis(
  			wasm_binary,
  			// Initial PoA authorities
  			vec![
  				authority_keys_from_seed("Alice"),
  			],
  			// Sudo account
  			get_account_id_from_seed::<sr25519::Public>("Alice"),
  			// Pre-funded accounts
  			vec![
  				get_account_id_from_seed::<sr25519::Public>("Alice"),
  				get_account_id_from_seed::<sr25519::Public>("Bob"),
  				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
  				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
  			],
  			true,
  		),
  		// Bootnodes
  		vec![],
  		// Telemetry
  		None,
  		// Protocol ID
  		None,
  		// Properties
  		None,
  		// Extensions
  		None,
  	))
  }
  
  
  pub fn local_testnet_config() -> Result<ChainSpec, String> {
  	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
  
  	Ok(ChainSpec::from_genesis(
  		// Name
  		"Local Testnet",
  		// ID
  		"local_testnet",
  		ChainType::Local,
  		move || testnet_genesis(
  			wasm_binary,
  			// Initial PoA authorities
  			vec![
  				authority_keys_from_seed("Alice"),
  				authority_keys_from_seed("Bob"),
  			],
  			// Sudo account
  			get_account_id_from_seed::<sr25519::Public>("Alice"),
  			// Pre-funded accounts
  			vec![
  				get_account_id_from_seed::<sr25519::Public>("Alice"),
  				get_account_id_from_seed::<sr25519::Public>("Bob"),
  				get_account_id_from_seed::<sr25519::Public>("Charlie"),
  				get_account_id_from_seed::<sr25519::Public>("Dave"),
  				get_account_id_from_seed::<sr25519::Public>("Eve"),
  				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
  				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
  				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
  				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
  				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
  				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
  				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
  			],
  			true,
  		),
  		// Bootnodes
  		vec![],
  		// Telemetry
  		None,
  		// Protocol ID
  		None,
  		// Properties
  		None,
  		// Extensions
  		None,
  	))
  }
  ```

  

  - 有更好的key的路径支持(Hierachical deterministic key derivations)
  - 本身支持集成多签名
  - 参考 [Polkadot wiki: sr25519](https://wiki.polkadot.network/docs/en/learn-keys#what-is-sr25519-and-where-did-it-come-from)
  - 参考 [Polkadot wiki: keypairs](https://wiki.polkadot.network/docs/en/learn-keys#what-is-sr25519-and-where-did-it-come-from)
