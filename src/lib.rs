//! [`std::concat!`] with support for `const` variables and expressions.
//!
//! Works on stable Rust ✨.
//!
//! # 🚀 Getting started
//!
//! Add `constcat` to your Cargo manifest.
//!
//! ```sh
//! cargo add constcat
//! ```
//!
//! Import the macro using the following.
//!
//! ```
//! use constcat::concat;
//! ```
//!
//! # 🤸 Usage
//!
//! ## String slices
//!
//! [`concat!`] works exactly like [`std::concat!`], concatenating [`&str`][str]
//! literals into a static string slice, except you can now pass variables and
//! constant expressions.
//!
//! ```
//! # use constcat::concat;
//! #
//! const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
//! const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
//! const fn tada() -> &'static str { "🎉" }
//! const VERSION: &str = concat!(CRATE_NAME, " ", CRATE_VERSION, tada());
//! ```
//!
//! ## Byte slices
//!
//! [`concat_bytes!`] works similarly to [`concat!`], concatenating `const`
//! [`&[u8]`][slice] expressions and literals into a static byte slice.
//!
//! ```
//! # use constcat::concat_bytes;
//! #
//! const VERSION: u32 = 1;
//! const fn entries() -> &'static [u8] { b"example" }
//! const HEADER: &[u8] = concat_bytes!(&VERSION.to_le_bytes(), entries());
//! ```
//!
//! ## `T` slices
//!
//! [`concat_slices!`] is the underlying macro used for both of the above, this
//! can be used to concatenate `const` [`&[T]`][slice] expressions into a static
//! slice.
//!
//! This macro requires the type of slice to be specified in the form `[T]: `
//! before the comma separated expressions.
//!
//! ```
//! # use constcat::concat_slices;
//! #
//! const MAGIC: &[i32; 4] = &[1, 3, 3, 7];
//! const VERSION: i32 = 1;
//! const HEADER: &[i32] = concat_slices!([i32]: MAGIC, &[0, VERSION]);
//! ```
//!
//! If the type is not a std integer, `f32`, `f64`, or `char` type then you must
//! also provide an initializer expression with the type, in the form `[init;
//! T]: `. This also works for custom types as long as the type and initializer
//! expression is able to be specified in an array initializer expression.
//!
//! ```
//! # use constcat::concat_slices;
//! #
//! const PRIMARIES: &'static [(u8, u8, u8)] = &[(255, 0, 0), (0, 255, 0), (0, 0, 255)];
//! const SECONDARIES: &'static [(u8, u8, u8)] = &[(255, 255, 0), (255, 0, 255), (0, 255, 255)];
//! const COLORS: &[(u8, u8, u8)] = concat_slices!([(0, 0, 0); (u8, u8, u8)]: PRIMARIES, SECONDARIES);
//! ```
//!
//! [`std::concat!`]: core::concat
//! [`std::concat_bytes!`]: core::concat_bytes

#![no_std]

#[doc(hidden)]
pub use core;

////////////////////////////////////////////////////////////////////////////////
// concat!
////////////////////////////////////////////////////////////////////////////////

/// Concatenate `const` [`&str`][str] expressions and literals into a static
/// string slice.
///
/// This macro takes any number of comma-separated literals or constant
/// expressions and yields an expression of type [`&'static str`][str] which is
/// the result of all of the literals and expressions concatenated
/// left-to-right. Literals are first converted using [`std::concat!`]. Finally,
/// each expression is converted to a byte slice and concatenated using
/// [`concat_slices!`].
///
/// See the [crate documentation][crate] for examples.
///
/// [`std::concat!`]: core::concat
#[macro_export]
macro_rules! concat {
    ($($e:expr),* $(,)?) => {
        $crate::_concat!($($e),*)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _concat {
    () => { "" };

    ($($maybe:expr),+) => {{
        $crate::_concat!(@impl $($crate::_maybe_std_concat!($maybe)),+)
    }};

    (@impl $($s:expr),+) => {{
        $(
            const _: &str = $s; // require str constants
        )*
        let slice: &[u8] = $crate::concat_slices!([u8]: $($s.as_bytes()),+);
        // SAFETY: The original constants were asserted to be &str's
        // so the resultant bytes are valid UTF-8.
        unsafe { $crate::core::str::from_utf8_unchecked(slice) }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _maybe_std_concat {
    ($e:literal) => {
        $crate::core::concat!($e)
    };
    ($e:expr) => {
        $e
    };
}

////////////////////////////////////////////////////////////////////////////////
// concat_bytes!
////////////////////////////////////////////////////////////////////////////////

/// Concatenate `const` [`&[u8]`][slice] expressions and literals into a static
/// byte slice.
///
/// This macro takes any number of comma-separated literals or constant
/// expressions and yields an expression of type [`&'static [u8]`][slice] which
/// is the result of all of the literals and expressions concatenated
/// left-to-right. Literals are first converted using [`std::concat_bytes!`].
/// Finally, each expression is concatenated using [`concat_slices!`].
///
/// See the [crate documentation][crate] for examples.
///
/// # Stability note
///
/// 🔬 This macro uses a nightly-only experimental API, [`std::concat_bytes!`],
/// for processing byte literals, until it is stabilized you will need to add
/// the following to the root of your crate. This is only required if you pass
/// any byte literals to the macro.
///
/// ```text
/// #![feature(concat_bytes)]
/// ```
///
/// # Differences to `std`
///
/// Unlike the standard library macro this macro does not accept byte array
/// literals directly like `[b'A', 32, b'B']` instead you have to pass a slice
/// like `&[b'A', 32, b'B']`.
///
/// [`std::concat_bytes!`]: core::concat_bytes
#[macro_export]
macro_rules! concat_bytes {
    ($($e:expr),* $(,)?) => {
        $crate::_concat_bytes!($($e),*)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _concat_bytes {
    () => { b"" };

    ($($maybe:expr),+) => {{
        $crate::_concat_bytes!(@impl $($crate::_maybe_std_concat_bytes!($maybe)),+)
    }};

    (@impl $($s:expr),+) => {{
        $crate::concat_slices!([u8]: $($s),+)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _maybe_std_concat_bytes {
    ($e:literal) => {
        $crate::core::concat_bytes!($e)
    };
    ($e:expr) => {
        $e
    };
}

////////////////////////////////////////////////////////////////////////////////
// concat_slices!
////////////////////////////////////////////////////////////////////////////////

/// Concatenate `const` [`&[T]`][slice] expressions into a static slice.
///
/// This macro takes any number of comma-separated [`&[T]`][slice] expressions
/// and yields an expression of type [`&'static [T]`][slice] which is the result
/// of all of the expressions concatenated left-to-right.
///
/// # Notes
///
/// - This macro requires that the type of slice be specified before the comma
///   separated expressions. This must be in the form `[T]: ` where `T` is the
///   the type.
///
///   ```
///   # use constcat::concat_slices;
///   concat_slices!([usize]: /* ... */);
///   ```
///
/// - If the type is not a std integer, `f32`, `f64`, or `char` type then you
///   must also provide an initializer expression.
///
///   ```
///   # use constcat::concat_slices;
///   concat_slices!([(0, 0, 0); (u8, u8, u8)]: /* ... */);
///   ```
/// - This also works for custom types as long as the type and initializer
///   expression is able to be specified in an array initializer expression.
///
///   ```
///   # use constcat::concat_slices;
///   #[derive(Clone, Copy)]
///   struct i256(i128, i128);
///
///   impl i256 {
///       const fn new() -> Self { Self(0, 0) }
///   }
///
///   concat_slices!([i256::new(); i256]: /* ... */);
///   ```
///
/// See the [crate documentation][crate] for examples.
#[macro_export]
macro_rules! concat_slices {
    ([$init:expr; $T:ty]: $($s:expr),* $(,)?) => {
        $crate::_concat_slices!([$init; $T]: $($s),*)
    };

    ([$T:ty]: $($s:expr),* $(,)?) => {
        $crate::concat_slices!([0 as $T; $T]: $($s),*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _concat_slices {
    ([$init:expr; $T:ty]:) => {{
        const ARR: [$T; 0] = [];
        &ARR
    }};

    ([$init:expr; $T:ty]: $($s:expr),+) => {{
        extern crate core;
        $(
            const _: &[$T] = $s; // require constants
        )*
        const LEN: usize = $( $s.len() + )* 0;
        const ARR: [$T; LEN] = {
            let mut arr: [$T; LEN] = unsafe {core::mem::MaybeUninit::zeroed().assume_init()};
            let mut base: usize = 0;
            $({
                let mut i = 0;
                while i < $s.len() {
                    arr[base + i] = $s[i];
                    i += 1;
                }
                base += $s.len();
            })*
            if base != LEN { panic!("invalid length"); }
            arr
        };
        &ARR
    }};
}
