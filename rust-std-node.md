# Rust Std Note

## trait From

```rust
/// Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
/// [`Into`].
///
/// One should always prefer implementing `From` over [`Into`]
/// because implementing `From` automatically provides one with an implementation of [`Into`]
/// thanks to the blanket implementation in the standard library.
///
/// Only implement [`Into`] when targeting a version prior to Rust 1.41 and converting to a type
/// outside the current crate.
/// `From` was not able to do these types of conversions in earlier versions because of Rust's
/// orphaning rules.
/// See [`Into`] for more details.
///
/// Prefer using [`Into`] over using `From` when specifying trait bounds on a generic function.
/// This way, types that directly implement [`Into`] can be used as arguments as well.
///
/// The `From` is also very useful when performing error handling. When constructing a function
/// that is capable of failing, the return type will generally be of the form `Result<T, E>`.
/// The `From` trait simplifies error handling by allowing a function to return a single error type
/// that encapsulate multiple error types. See the "Examples" section and [the book][book] for more
/// details.
///
/// **Note: This trait must not fail**. If the conversion can fail, use [`TryFrom`].
///
/// # Generic Implementations
///
/// - `From<T> for U` implies [`Into`]`<U> for T`
/// - `From` is reflexive, which means that `From<T> for T` is implemented
///
/// # Examples
///
/// [`String`] implements `From<&str>`:
///
/// An explicit conversion from a `&str` to a String is done as follows:
///
/// ```
/// let string = "hello".to_string();
/// let other_string = String::from("hello");
///
/// assert_eq!(string, other_string);
/// ```
///
/// While performing error handling it is often useful to implement `From` for your own error type.
/// By converting underlying error types to our own custom error type that encapsulates the
/// underlying error type, we can return a single error type without losing information on the
/// underlying cause. The '?' operator automatically converts the underlying error type to our
/// custom error type by calling `Into<CliError>::into` which is automatically provided when
/// implementing `From`. The compiler then infers which implementation of `Into` should be used.
///
/// ```
/// use std::fs;
/// use std::io;
/// use std::num;
///
/// enum CliError {
///     IoError(io::Error),
///     ParseError(num::ParseIntError),
/// }
///
/// impl From<io::Error> for CliError {
///     fn from(error: io::Error) -> Self {
///         CliError::IoError(error)
///     }
/// }
///
/// impl From<num::ParseIntError> for CliError {
///     fn from(error: num::ParseIntError) -> Self {
///         CliError::ParseError(error)
///     }
/// }
///
/// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
///     let mut contents = fs::read_to_string(&file_name)?;
///     let num: i32 = contents.trim().parse()?;
///     Ok(num)
/// }
/// ```
///
/// [`Option<T>`]: Option
/// [`Result<T, E>`]: Result
/// [`String`]: ../../std/string/struct.String.html
/// [`from`]: From::from
/// [book]: ../../book/ch09-00-error-handling.html
#[rustc_diagnostic_item = "from_trait"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(on(
    all(_Self = "&str", T = "std::string::String"),
    note = "to coerce a `{T}` into a `{Self}`, use `&*` as a prefix",
))]
pub trait From<T>: Sized {
    /// Performs the conversion.
    #[lang = "from"]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn from(_: T) -> Self;
}
```

## Triat Into 

```rust
/// A value-to-value conversion that consumes the input value. The
/// opposite of [`From`].
///
/// One should avoid implementing [`Into`] and implement [`From`] instead.
/// Implementing [`From`] automatically provides one with an implementation of [`Into`]
/// thanks to the blanket implementation in the standard library.
///
/// Prefer using [`Into`] over [`From`] when specifying trait bounds on a generic function
/// to ensure that types that only implement [`Into`] can be used as well.
///
/// **Note: This trait must not fail**. If the conversion can fail, use [`TryInto`].
///
/// # Generic Implementations
///
/// - [`From`]`<T> for U` implies `Into<U> for T`
/// - [`Into`] is reflexive, which means that `Into<T> for T` is implemented
///
/// # Implementing [`Into`] for conversions to external types in old versions of Rust
///
/// Prior to Rust 1.41, if the destination type was not part of the current crate
/// then you couldn't implement [`From`] directly.
/// For example, take this code:
///
/// ```
/// struct Wrapper<T>(Vec<T>);
/// impl<T> From<Wrapper<T>> for Vec<T> {
///     fn from(w: Wrapper<T>) -> Vec<T> {
///         w.0
///     }
/// }
/// ```
/// This will fail to compile in older versions of the language because Rust's orphaning rules
/// used to be a little bit more strict. To bypass this, you could implement [`Into`] directly:
///
/// ```
/// struct Wrapper<T>(Vec<T>);
/// impl<T> Into<Vec<T>> for Wrapper<T> {
///     fn into(self) -> Vec<T> {
///         self.0
///     }
/// }
/// ```
///
/// It is important to understand that [`Into`] does not provide a [`From`] implementation
/// (as [`From`] does with [`Into`]). Therefore, you should always try to implement [`From`]
/// and then fall back to [`Into`] if [`From`] can't be implemented.
///
/// # Examples
///
/// [`String`] implements [`Into`]`<`[`Vec`]`<`[`u8`]`>>`:
///
/// In order to express that we want a generic function to take all arguments that can be
/// converted to a specified type `T`, we can use a trait bound of [`Into`]`<T>`.
/// For example: The function `is_hello` takes all arguments that can be converted into a
/// [`Vec`]`<`[`u8`]`>`.
///
/// ```
/// fn is_hello<T: Into<Vec<u8>>>(s: T) {
///    let bytes = b"hello".to_vec();
///    assert_eq!(bytes, s.into());
/// }
///
/// let s = "hello".to_string();
/// is_hello(s);
/// ```
///
/// [`Option<T>`]: Option
/// [`Result<T, E>`]: Result
/// [`String`]: ../../std/string/struct.String.html
/// [`Vec`]: ../../std/vec/struct.Vec.html
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Into<T>: Sized {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn into(self) -> T;
}
```

