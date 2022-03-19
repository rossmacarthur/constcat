#![no_std]

//! Provides a [`constcat!`] macro like [`std::concat!`] but with support for
//! `const` expressions that return [`&str`][str].
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
        $crate::__constcat!($($e),*)
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __constcat {
    ($($maybe:expr),*) => {{
        $crate::__constcat!(impl $(
            $crate::__maybe_concat!($maybe)
        ),* )
    }};
    (impl $($s:expr),*) => {{
        #[allow(unused_mut)] // for the empty case
        const fn gen() -> &'static str {
            $(
                const _: &str = $s; // require constants
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
                    panic!("invalid length written");
                }
                arr
            };
            // SAFETY: The original constants were asserted to be &str's
            // so the resultant bytes are valid UTF-8.
            unsafe { core::mem::transmute::<&[u8], &str>(&ARR) }
        }
        gen()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __maybe_concat {
    ($e:literal) => {
        concat!($e)
    };
    ($e:expr) => {
        $e
    };
}

#[doc(hidden)]
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

        const TEST2: &str = constcat!("one");
        assert_eq!(TEST2, "one");

        const TEST3: &str = constcat!("one", 2);
        assert_eq!(TEST3, "one2");

        const TEST4: &str = constcat!("before ", TEST3, " after");
        assert_eq!(TEST4, "before one2 after");

        const TEST5: &str = constcat!("before ", env!("CARGO_PKG_NAME"), " after");
        assert_eq!(TEST5, "before constcat after");
    }
}
