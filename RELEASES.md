# 📝 Release notes

## 0.6.1

*May 18th, 2025*

- [Simplify internal concat implementation][3d0e98ad]. Removes some unnecessary
  `unsafe` code.

[3d0e98ad]: https://github.com/rossmacarthur/constcat/commit/3d0e98adac4c1716147917ec1c109a2d1c8cccf1

## 0.6.0

*January 28th, 2025*

- [Improve macro performance][ad3cb1a1]. Improves performance of the macro
  particularly when concatenating a large number of strs/slices. This change
  requires the MSRV to be increased to Rust 1.66.

[ad3cb1a1]: https://github.com/rossmacarthur/constcat/commit/ad3cb1a15bd5c2b718b60f8da15b2e943c645312

## 0.5.1

*October 5th, 2024*

- [Reduce MSRV to Rust 1.60][fffa1fb8]. Removes usage of `MaybeUninit::zeroed()`
  which lowers the MSRV to Rust 1.60.

[fffa1fb8]: https://github.com/rossmacarthur/constcat/commit/fffa1fb8a250fe768d5fdf1e4f8041e3d6400d76

## 0.5.0

*February 29th, 2024*

- [Allow slice concatenation without initial value][d0ceefed]. This change
  removes the need to add the initializer for non-number types when calling
  `concat_slices!`. Instead of having to write something like this:
  ```rust
  concat_slices!([(0, 0, 0); (u8, u8, u8)]: /* ... */);
  ```
  You can just write
  ```rust
  concat_slices!([(u8, u8, u8)]: /* ... */);
  ```
  *Contributed by [**airblast-dev**](https://github.com/airblast-dev)*

[d0ceefed]: https://github.com/rossmacarthur/constcat/commit/d0ceefedd2cd20f0bb80faaeddd0d6971d91002e

## 0.4.1

*January 24th, 2024*

- [Remove special casing on primitive types by casting 0 to type][ae09191f]

  *Contributed by [**airblast-dev**](https://github.com/airblast-dev)*

[ae09191f]: https://github.com/rossmacarthur/constcat/commit/ae09191f487b16bf997e5c23dc7dba261c9381ff

## 0.4.0

*November 3rd, 2023*

- [Remove `bytes` feature, make `concat_bytes!` always available][b7311dcf]

[b7311dcf]: https://github.com/rossmacarthur/constcat/commit/b7311dcfb8a1baa6288d2f970812b60368bb3d4c

## 0.3.1

*October 10th, 2023*

- [Special case initializers for `f32`, `f64`, `char`][66a2927a]

[66a2927a]: https://github.com/rossmacarthur/constcat/commit/66a2927a38270d28ac0246c8699f21f93573543c

## 0.3.0

*January 4th, 2023*

- [Make `concat_slices!` macro public][2c97f45b]
- [Add `bytes` feature flag][34a4c5ae]
- [Add `concat_bytes!` macro][0a4477bb]

[2c97f45b]: https://github.com/rossmacarthur/constcat/commit/2c97f45b38b9937ec5043c423a33ccd411b4503d
[34a4c5ae]: https://github.com/rossmacarthur/constcat/commit/34a4c5ae624b65cf5df98b5a17592532bc1763fe
[0a4477bb]: https://github.com/rossmacarthur/constcat/commit/0a4477bb67281623088a215888938b957a86275f

## 0.2.0

*September 25th, 2022*

- [Fix even more possible namespacing issues][254bb31b]
- [Rename macro to `concat!`][c05eaa70]
- [Improve examples in README][4220875a]

[254bb31b]: https://github.com/rossmacarthur/constcat/commit/254bb31ba09a46903c5d68f51e7c8b9b837bc6be
[c05eaa70]: https://github.com/rossmacarthur/constcat/commit/c05eaa70ecf7a640f1b180e6f0586f3cbc2204de
[4220875a]: https://github.com/rossmacarthur/constcat/commit/4220875a5fd0a1b7aa3d94eff90e40463f6a6315

## 0.1.1

*March 30th, 2022*

- [Fix namespacing issues][2c3341bb].

[2c3341bb]: https://github.com/rossmacarthur/constcat/commit/2c3341bb2b721a83c2ba7cdead54b0f573b1c1b7

## 0.1.0

*March 19th, 2022*

First release.
