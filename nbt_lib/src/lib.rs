pub mod error;
pub mod traits;

mod data;
pub use data::{NbtData, NbtValue, NbtTypeId};

#[cfg(test)]
mod test;
