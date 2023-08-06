//! Binn format with Rust
//!
//! binn-rs features no_alloc support and aims to provide better
//! performance for embedded devices by using static allocations and
//! providing zero-copy read operations
//!
//! # Quick start
//!
//! ```
//! use binn_rs::{Object, Value};
//!
//! let mut buf = [0; 32];
//! // create new object, that will use `buf` for storage
//! // obj will have same lifetime as it's storage
//! let mut obj = Object::empty_mut(buf.as_mut_slice()).unwrap();
//!
//! // add some values
//! obj.add_value("key1", false).unwrap();
//! obj.add_value("key2", 6262u16).unwrap();
//!
//! // get serialized representation
//! let serialized = obj.as_bytes();
//!
//! let expected = &[
//!     0xE2,                          // [type] object (container)
//!     0x11,                          // [size] container total size
//!     0x02,                          // [count] key/value pairs
//!     
//!     0x04, b'k', b'e', b'y', b'1',  // key
//!     0x02,                          // [type] = false
//!     
//!     0x04, b'k', b'e', b'y', b'2',  // key
//!     0x40,                          // [type] = uint16
//!     0x18, 0x76                     // [data] (6262)
//! ];
//!
//! assert_eq!(serialized, expected);
//!
//! // deserialize binn Value
//! let value = Value::deserialize(serialized).unwrap();
//! // unwrap Object from value
//! let obj: Object = value.try_into().unwrap();
//!
//! // check object contents
//! let expected = vec![
//!     ("key1", Value::False),
//!     ("key2", Value::UInt16(6262)),
//! ];
//!
//! assert_eq!(expected.len(), obj.count());
//!
//! for ((ref actual_key, ref actual_val), (expected_key, expected_val)) in
//!     obj.iter().zip(expected.iter())
//! {
//!     assert_eq!(actual_key, expected_key);
//!     assert_eq!(actual_val, expected_val);
//! }
//!
//! ```
//!
#![no_std]
#![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]

/// Module contains possible error types
pub mod error;

mod allocation;
mod container;
mod data_type;
mod raw_container;
mod size;
mod storage;
mod subtype;
mod utils;
mod value;

pub use allocation::Allocation;
pub use container::{List, Map, Object};
pub use subtype::SubType;
pub use value::Value;
