# Substrate Recipes Zh

## Storage Maps 

在这个recipe中，我们将看到如何存储一个mapping从键到值的一个映射，类似于Rust自身的HashMap.

### 声明一个StorageMap

我们可以使用以下语法声明一个存储映射

```rust
decl_storage! {
  trait Store for Module<T: Trait> as SimpleMap {
    SimpleMap get(fn simple_map) : map hasher(blake2_128_concat) T::AccountId => u32;
  }
}
```

从存储的值来看，大部分的内容应该很熟悉

- SimpleMap - 存储map的名称
- Get(fn simple_map) - the name of a getter function that will return values from the map 
- : map hasher(blake2_128_concat)
- T::AccountId => u32

### 选择一个Hasher

- Blake2_128_concat
- Twox_64_concat
- identity

### Storage Map 的API 

```rust
//Insert 
<SimpleMap<T>>::insert(&user, entry);

//Get 
let entry = <SimpleMap<T>>::get(account);

// Take 
let entry = <SimpleMap<T>>::take(&user);

// Contains key 
<SimpleMap<T>>::contains_key(&user)
```

