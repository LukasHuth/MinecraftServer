macro_rules! generate_add_function {
    ($($t:ty$(,)?)*) => {        
        $(paste::paste! {
            #[doc="Appends an [`" $t "`] to the end of a list of bytes"]
            pub fn [< add_ $t >](value: $t, arr: &mut Vec<u8>) {
                arr.extend_from_slice(&value.to_be_bytes())
            }
        })*
        paste::paste! {
            #[doc="Appends an [`str`] to the end of a list of bytes"]
            pub fn [< add_ str >](value: &str, arr: &mut Vec<u8>) -> Result<(), ()> {
                if value.len() > u16::MAX as usize { return Err(()); }
                arr.extend_from_slice(&(value.len() as u16).to_be_bytes());
                Ok(arr.extend_from_slice(value.as_bytes()))
            }
        }
        paste::paste! {
            #[doc="Appends an [`[u8]`] to the end of a list of bytes"]
            pub fn [< add_ bytes >](value: &[u8], arr: &mut Vec<u8>) -> Result<(), ()> {
                if value.len() > i32::MAX as usize { return Err(()); }
                arr.extend_from_slice(&(value.len() as u16).to_be_bytes());
                Ok(arr.extend_from_slice(value))
            }
        }
    };
}
generate_add_function!(i8, i16, u16, i32, i64, f32, f64);
