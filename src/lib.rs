//! [`std::concat!`][core::concat!] with support for `const` variables and expressions.
//!
//! # Examples
//!
//! ```rust
//! use constcat::constcat;
//!
//! const EX: &str = constcat!("string", 10, 'c', true, 3.14, VARIABLE, expr());
//! assert_eq!(EX, "string10ctrue3.14constcat🎉");
//!
//! const VARIABLE: &str = env!("CARGO_PKG_NAME");
//!
//! const fn expr() -> &'static str {
//!     "🎉"
//! }
//! ```

#![no_std]

/// Concatenate [`&str`][str] `const` expressions and literals into a static
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
macro_rules! constcat {
    ($($e:expr),* $(,)?) => {{
        $crate::_constcat!($($e),*)
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! _constcat {
    () => {
        ""
    };

    ($($maybe:expr),+) => {{
        $crate::_constcat!(@impl $($crate::_maybe_concat!($maybe)),*)
    }};

    (@impl $($s:expr),+) => {{
        $(
            const _: &str = $s; // require str constants
        )*
        const LEN: usize = $( $s.len() + )* 0;
        const ARR: [u8; LEN] = {
            let mut arr = [0u8; LEN];
            let mut off = 0usize;
            $(
                arr = $crate::copy_into(arr, off, $s.as_bytes());
                off += $s.len();
            )*
            if off != LEN {
                ::core::panic!("invalid length written");
            }
            arr
        };
        // SAFETY: The original constants were asserted to be &str's
        // so the resultant bytes are valid UTF-8.
        unsafe { ::core::str::from_utf8_unchecked(&ARR) }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _maybe_concat {
    ($e:literal) => {
        ::core::concat!($e)
    };
    ($e:expr) => {
        $e
    };
}

#[doc(hidden)]
#[allow(clippy::all)]
pub const fn copy_into<const N: usize>(mut into: [u8; N], offset: usize, from: &[u8]) -> [u8; N] {
    let mut i = 0;
    loop {
        if i == from.len() {
            break;
        }
        into[offset + i] = from[i];
        i += 1;
    }
    into
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        const TEST0: &str = constcat!("test", 10, 'b', true);
        assert_eq!(TEST0, "test10btrue");

        const TEST1: &str = constcat!();
        assert_eq!(TEST1, "");

        const TEST2: &str = constcat!(,);
        assert_eq!(TEST2, "");

        const TEST3: &str = constcat!("one");
        assert_eq!(TEST3, "one");

        const TEST4: &str = constcat!("one",);
        assert_eq!(TEST4, "one");

        const TEST5: &str = constcat!("one", 2);
        assert_eq!(TEST5, "one2");

        const TEST6: &str = constcat!("before ", TEST5, " after");
        assert_eq!(TEST6, "before one2 after");

        const TEST7: &str = constcat!("before ", env!("CARGO_PKG_NAME"), " after");
        assert_eq!(TEST7, "before constcat after");
    }

    #[test]
    fn namespacing() {
        #[allow(unused_imports)]
        use core::array as core;

        macro_rules! _maybe_concat {
            () => {};
        }

        const TEST0: &str = constcat!("test", 10, 'b', true);
        assert_eq!(TEST0, "test10btrue");
    }
}
