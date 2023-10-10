#![cfg_attr(feature = "bytes", feature(concat_bytes))]

#[test]
fn concat_smoke() {
    use constcat::concat;

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
    use constcat::concat_bytes;

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
fn concat_slices_smoke() {
    use constcat::concat_slices;

    const TEST0: &[i32] = concat_slices!([i32]: &[1, 2, 3]);
    assert_eq!(TEST0, [1, 2, 3]);

    const TEST1: &[i32] = concat_slices!([i32]: &[1, 2, 3],);
    assert_eq!(TEST1, [1, 2, 3]);

    const TEST2: &[i32] = concat_slices!([i32]: &[1, 2, 3], TEST1);
    assert_eq!(TEST2, [1, 2, 3, 1, 2, 3]);

    const TEST3: &[f32] = concat_slices!([f32]: &[1.], &[2.], &[3.]);
    assert_eq!(TEST3, [1., 2., 3.]);

    const TEST4: &[char] = concat_slices!([char]: &['a'], &['b'], &['c']);
    assert_eq!(TEST4, ['a', 'b', 'c']);

    const TEST5: &[f32] = concat_slices!([1.0; f32]: &[1.], &[2.], &[3.]);
    assert_eq!(TEST5, [1., 2., 3.]);

    const TEST6: &[char] = concat_slices!(['0'; char]: &['a'], &['b'], &['c']);
    assert_eq!(TEST6, ['a', 'b', 'c']);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct I(i32);
    const TEST7: &[I] = concat_slices!([I(0); I]: &[I(1), I(2), I(3)]);
    assert_eq!(TEST7, [I(1), I(2), I(3)]);

    const DEF: I = I(123);
    const TEST8: &[I] = concat_slices!([DEF; I]: &[I(1), I(2), I(3)]);
    assert_eq!(TEST8, [I(1), I(2), I(3)]);
}

#[test]
fn concat_namespacing() {
    use constcat::concat;

    #[allow(unused_imports)]
    use core::array as core;

    macro_rules! _maybe_concat {
        () => {};
    }

    const TEST0: &str = concat!("test", 10, 'b', true);
    assert_eq!(TEST0, "test10btrue");
}
