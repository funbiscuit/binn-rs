## binn-rs

[<img alt="github" src="https://img.shields.io/badge/github-funbiscuit/binn--rs-a?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/funbiscuit/binn-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/binn-rs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/binn-rs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-binn--rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/binn-rs)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/funbiscuit/binn-rs/ci.yaml?branch=master&style=for-the-badge" height="20">](https://github.com/funbiscuit/binn-rs/actions/workflows/ci.yaml)
[<img alt="code coverage" src="https://img.shields.io/codecov/c/github/funbiscuit/binn-rs?style=for-the-badge" height="20">](https://codecov.io/github/funbiscuit/binn-rs)

Small [binn](https://github.com/liteserver/binn) implementation with no_std/no_alloc support.

API is not quite stable so breaking changes are possible.
Any breaking change will lead to bump in major version (e.g. `0.1.0` -> `0.2.0`).

## Features

- [x] Static allocation
- [ ] Dynamic allocation (`alloc` feature)
- [x] User defined primitives support
- [x] Zero copy (for read operations)

## Data type support

| Data type                                                 | Supported |
|-----------------------------------------------------------|:---------:|
| null                                                      |     ✓     |
| boolean (`true` and `false`)                              |     ✓     |
| integer (up to 64 bits signed or unsigned)                |     ✓     |
| floating point numbers (IEEE single and double precision) |     ✓     |
| string                                                    |     ✓     |
| blob (binary data)                                        |     ✓     |
| user defined primitive                                    |     ✓     |
| list                                                      |     ✓     |
| map (numeric key associative array)                       |     ✓     |
| object (text key associative array)                       |     ✓     |

## Limitations

* Containers can be only of predefined types (list, map and object), user types
  are not supported for containers

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
