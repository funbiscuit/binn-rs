## binn-rs

Small [binn](https://github.com/liteserver/binn) implementation with no_std/no_alloc support.

API is not quite stable so breaking changes are possible.
Any breaking change will lead to bump in major version (e.g. `0.1.0` -> `0.2.0`).

## Features

* Static allocation
* Dynamic allocation (`alloc` feature)
* User defined primitives support
* Zero copy (for read operations)

## Data type support

| Data type                                                 | Supported |
|-----------------------------------------------------------|:---------:|
| null                                                      |     +     |
| boolean (`true` and `false`)                              |     +     |
| integer (up to 64 bits signed or unsigned)                |     +     |
| floating point numbers (IEEE single and double precision) |     +     |
| string                                                    |     +     |
| blob (binary data)                                        |     +     |
| user defined primitive                                    |     +     |
| list                                                      |     +     |
| map (numeric key associative array)                       |     +     |
| object (text key associative array)                       |     +     |

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
