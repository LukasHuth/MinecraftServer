#![feature(buf_read_has_data_left)]
//! This is a crate that Provides all the Packets, that the minecraft Protocol uses.
#![deny(missing_docs)]

/// Module that provides all Packets as Datatypes
pub mod datatypes;
mod traits {
    use binary_utils::DataWriter;

    pub trait PacketWrite: Sized + DataWriter {
        fn write(
            &self,
            writer: &mut (impl tokio::io::AsyncWrite + Unpin),
        ) -> impl std::future::Future<Output = binary_utils::Result<()>>;
        fn get_id(&self) -> i32;
    }
}
