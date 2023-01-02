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
#![cfg_attr(feature = "bytes", feature(concat_bytes))]

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
///
/// Integer, floating point, and boolean literals are stringified in order to be
/// concatenated.
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
        let slice: &[u8] = $crate::_concat_slices!(&[u8]: $($s.as_bytes()),+);
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
///
/// See the [crate documentation][crate] for examples.
///
/// # Note
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
/// literals directly like `[b'A', 32, b'B']` instead you have to pass a slice like
/// `&[b'A', 32, b'B']`.
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
        $crate::_concat_slices!(&[u8]: $($s),+)
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
// Utilities
////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! _concat_slices {
    (&[$ty:ty]: $($s:expr),+) => {{
        $(
            const _: &[$ty] = $s; // require constants
        )*
        const LEN: usize = $( $s.len() + )* 0;
        const ARR: [$ty; LEN] = {
            let mut arr: [$ty; LEN] = [0; LEN];
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
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_smoke() {
        const TEST0: &str = concat!("test", 10, 'b', true);
        assert_eq!(TEST0, "test10btrue");

        const TEST1: &str = concat!();
        assert_eq!(TEST1, "");

        const TEST2: &str = concat!(,);
        assert_eq!(TEST2, "");

        const TEST3: &str = concat!("one");
        assert_eq!(TEST3, "one");

        const TEST4: &str = concat!("one",);
        assert_eq!(TEST4, "one");

        const TEST5: &str = concat!("one", 2);
        assert_eq!(TEST5, "one2");

        const TEST6: &str = concat!("before ", TEST5, " after");
        assert_eq!(TEST6, "before one2 after");

        const TEST7: &str = concat!("before ", env!("CARGO_PKG_NAME"), " after");
        assert_eq!(TEST7, "before constcat after");
    }

    #[test]
    #[cfg(feature = "bytes")]
    fn concat_bytes_smoke() {
        const TEST0: &[u8] = concat_bytes!(b"test", b'b', &[68, b'E', 70]);
        assert_eq!(TEST0, b"testbDEF");

        const TEST1: &[u8] = concat_bytes!();
        assert_eq!(TEST1, b"");

        const TEST2: &[u8] = concat_bytes!(,);
        assert_eq!(TEST2, b"");

        const TEST3: &[u8] = concat_bytes!(b"one");
        assert_eq!(TEST3, b"one");

        const TEST4: &[u8] = concat_bytes!(b"one",);
        assert_eq!(TEST4, b"one");

        const TEST5: &[u8] = concat_bytes!(b"one", b'2');
        assert_eq!(TEST5, b"one2");

        const TEST6: &[u8] = concat_bytes!(b"before ", TEST5, b" after");
        assert_eq!(TEST6, b"before one2 after");
    }

    #[test]
    fn concat_namespacing() {
        #[allow(unused_imports)]
        use core::array as core;

        macro_rules! _maybe_concat {
            () => {};
        }

        const TEST0: &str = concat!("test", 10, 'b', true);
        assert_eq!(TEST0, "test10btrue");
    }
}
