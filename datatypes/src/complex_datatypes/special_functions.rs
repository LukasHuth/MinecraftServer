use binary_utils::{DataReader, DataWriter};

use crate::Array;

impl<T> Array<T>
where
    T: DataReader + DataWriter,
{
    /// returns the length of the array
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
