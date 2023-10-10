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
//! [`concat!`] works exactly like [`std::concat!`] except you can
//! now pass variables and constant expressions. For example:
//!
//! ```
//! # use constcat::concat;
//! #
//! const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
//! const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
//! const fn tada() -> &'static str { "🎉" }
//! const VERSION: &str = concat!(CRATE_NAME, " ", CRATE_VERSION, tada());
//! #
//! # assert_eq!(
//! #     VERSION,
//! #     std::concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), "🎉"),
//! # );
//! ```
//!
//! [`concat_bytes!`] works similarly except it yields a static byte slice. For
//! example:
//!
//! ```
//! # use constcat::concat_bytes;
//! #
//! const VERSION: u32 = 1;
//! const fn entries() -> &'static [u8] { b"example" }
//! const HEADER: &[u8] = concat_bytes!(&VERSION.to_le_bytes(), entries());
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
/// expressions and yields an expression of type [`&'static str`][str] which
/// represents all of the literals and expressions concatenated left-to-right.
/// Integer, floating point, and boolean literals are stringified in order to be
/// concatenated. Finally, each expression is converted to a byte slice and
/// concatenated using [`concat_slices!`].
///
/// See the [crate documentation][crate] for examples.
#[macro_export]
macro_rules! concat {
    ($($e:expr),* $(,)?) => {{
        $crate::_concat!($($e),*)
    }}
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
/// represents all of the literals and expressions concatenated left-to-right.
/// Literals are converted using [`core::concat_bytes!`] and then each
/// expression is concatenated using [`concat_slices!`].
///
/// See the [crate documentation][crate] for examples.
///
/// # Stability note
///
/// 🔬 This macro uses a nightly-only experimental API, [`core::concat_bytes`],
/// for processing byte literals, until it is stabilized you will need to add
/// the following to the root of your crate.
///
/// ```
/// #![feature(concat_bytes)]
/// ```
///
/// Unlike the standard library macro this macro does not accept byte array
/// literals directly like `[b'A', 32, b'B']` instead you have to pass a slice
/// like `&[b'A', 32, b'B']`.
#[macro_export]
#[cfg(feature = "bytes")]
macro_rules! concat_bytes {
    ($($e:expr),* $(,)?) => {{
        $crate::_concat_bytes!($($e),*)
    }}
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "bytes")]
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
#[cfg(feature = "bytes")]
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
/// - This macro takes any number of comma-separated [`&[T]`][slice] expressions
///   and yields an expression of type [`&'static [T]`][slice] which represents
///   all of the expressions concatenated left-to-right.
/// - The macro requires that type of slice be specified, e.g. `[usize]` or
///   `[u8]` before the comma separate expressions.
/// - You can optionally provide an initializer for non-integer types, e.g.
///   `[0.0; f32]` for floating point numbers, `[false; bool]` for `bool`s, or
///   `['\x00'; char]` for `char`s. This also works for custom types as long as
///   the type and initializer expression is able to be specified in an array
///   initializer expression.
///
/// # Examples
///
/// Basic usage with integers:
///
/// ```
/// # use constcat::concat_slices;
/// #
/// const fn more() -> &'static [i32] { &[4, 5, 6] }
/// const EXAMPLE: &[i32] = concat_slices!([i32]: &[1, 2, 3], more());
/// assert_eq!(EXAMPLE, [1, 2, 3, 4, 5, 6])
/// ```
///
/// With a constant initializer:
///
/// ```
/// # use constcat::concat_slices;
/// #
/// const fn more() -> &'static [f32] { &[4.0, 5.0, 6.0] }
/// const EXAMPLE: &[f32] = concat_slices!([0.0; f32]: &[1.0, 2.0, 3.0], more());
/// assert_eq!(EXAMPLE, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
/// ```
#[macro_export]
macro_rules! concat_slices {
    ([$init:expr; $T:ty]: $($s:expr),+ $(,)?) => {{
        $(
            const _: &[$T] = $s; // require constants
        )*
        const LEN: usize = $( $s.len() + )* 0;
        const ARR: [$T; LEN] = {
            let mut arr: [$T; LEN] = [$init; LEN];
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

    ([char]: $($s:expr),+ $(,)?) => {
        $crate::concat_slices!(['\x00'; char]: $($s),+)
    };

    ([f32]: $($s:expr),+ $(,)?) => {
        $crate::concat_slices!([0.0; f32]: $($s),+)
    };

    ([f64]: $($s:expr),+ $(,)?) => {
        $crate::concat_slices!([0.0; f64]: $($s),+)
    };

    ([$T:ty]: $($s:expr),+ $(,)?) => {
        $crate::concat_slices!([0; $T]: $($s),+)
    };
}
