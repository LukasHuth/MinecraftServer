//! This module contains a serde implementation to serialize into NBT data.
//!
//! # Shoutout
//!
//! Big thanks to [Owen Gage](https://github.com/owengage) and their project [fastnbt](https://github.com/owengage/fastnbt/tree/master)
//! it helped so much at implementing this version
//!
//! # To Do
//! The structs and implementations inside of this module should get their own modules and files
//!
//! # Example
//! ```
//! use serde::Serialize;
//!
//! use nbt_lib::ser::to_bytes_with_opts;
//!
//! #[derive(Serialize, Debug)]
//! struct HelloWorld {
//!     name: String
//! }
//!
//! #[test]
//! fn test_ser() {
//!     let test = HelloWorld { name: "Bananrama".to_string() };
//!     let test_data = to_bytes_with_opts(&test, "hello world".into()).unwrap();
//!     let expected_data = include_bytes!("../test_data/hello_world.nbt");
//!     assert_eq!(test_data, expected_data);
//! }
//! ```

mod write_nbt_trait;
// use write_nbt_trait::WriteNbt;

mod name_serializer;

/// Module fot the basic nbt serializer
mod serializer;
pub use serializer::Serializer;

mod conversion_functions;
pub use conversion_functions::*;
