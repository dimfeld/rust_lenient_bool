# LenientBool

[![crates.io](https://img.shields.io/crates/v/lenient_bool.svg)](https://crates.io/crates/lenient_bool)

This crate provides the LenientBool type, which performs the following string -> bool conversions:

* true, t, yes, y, and 1 to `true`
* false, f, no, n, and 0 to `false`

Comparisons are case-insensitive, so `TRUE`, `tRue`, and `T` all work, for example.

[Documentation](https://docs.rs/lenient_bool/)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
