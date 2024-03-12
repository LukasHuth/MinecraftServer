use crate::datatypes::{VarInt, VarLong, necesary::Necesary};

macro_rules! assert_eq_var_int {
    ($exp:expr, $expected:expr) => {
        let arr = $exp;
        let mut vl = VarInt::new(0);
        vl.read(&mut arr.iter(), None);
        assert_eq!(vl.get_value(), $expected);
        
    };
}
macro_rules! assert_eq_var_long {
    ($exp:expr, $expected:expr) => {
        let arr = $exp;
        let mut vl = VarLong::new(0);
        vl.read(&mut arr.iter(), None);
        assert_eq!(vl.get_value(), $expected);
        
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_var_int() {
        assert_eq_var_int!([0x00], 0);
        assert_eq_var_int!([0x01], 1);
        assert_eq_var_int!([0x02], 2);
        assert_eq_var_int!([0x7f], 127);
        assert_eq_var_int!([0x80, 0x01], 128);
        assert_eq_var_int!([0xff, 0x01], 255);
        assert_eq_var_int!([0xdd, 0xc7, 0x01], 25565);
        assert_eq_var_int!([0xff, 0xff, 0x7f], 2097151);
        assert_eq_var_int!([0xff, 0xff, 0xff, 0xff, 0x07], 2147483647);
        assert_eq_var_int!([0xff, 0xff, 0xff, 0xff, 0x0f], -1);
        assert_eq_var_int!([0x80, 0x80, 0x80, 0x80, 0x08], -2147483648);
        //let vl = VarInt::new(0);
    }
    #[test]
    pub fn test_var_long() {
        assert_eq_var_long!([0x00], 0);
        assert_eq_var_long!([0x01], 1);
        assert_eq_var_long!([0x02], 2);
        assert_eq_var_long!([0x7f], 127);
        assert_eq_var_long!([0x80, 0x01], 128);
        assert_eq_var_long!([0xff, 0x01], 255);
        assert_eq_var_long!([0xdd, 0xc7, 0x01], 25565);
        assert_eq_var_long!([0xff, 0xff, 0x7f], 2097151);
        assert_eq_var_long!([0xff, 0xff, 0xff, 0xff, 0x07], 2147483647);
        assert_eq_var_long!([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f], 9223372036854775807);
        assert_eq_var_long!([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01], -1);
        assert_eq_var_long!([0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01], -2147483648);
        assert_eq_var_long!([0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01], -9223372036854775808);
    }
}
