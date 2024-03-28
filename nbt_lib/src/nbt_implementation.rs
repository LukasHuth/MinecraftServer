use crate::{NbtValue, error::{NbtResult, NbtError}, traits::{NbtRead, NbtWrite}, reader::NbtReader, NbtTypeId};

impl NbtValue {
    pub fn from_binary<R>(data: Vec<u8>) -> NbtResult<NbtValue> where R: NbtRead {
        let reader = NbtReader::new(data);
        R::from_reader(reader)
    }
    pub fn tag(&self) -> NbtTypeId {
        match self {
            NbtValue::Byte(_) => 1,
            NbtValue::Short(_) => 2,
            NbtValue::Int(_) => 3,
            NbtValue::Long(_) => 4,
            NbtValue::Float(_) => 5,
            NbtValue::Double(_) => 6,
            NbtValue::ByteArray(_) => 7,
            NbtValue::String(_) => 8,
            NbtValue::List(_) => 9,
            NbtValue::Compound(_, _) => 10,
            NbtValue::IntArray(_) => 11,
            NbtValue::LongArray(_) => 12,
        }
    }
    pub fn write_to<W>(&self, buff: &mut Vec<u8>) -> NbtResult<()> where W: NbtWrite {
        W::write_to(self, buff)
    }
    pub fn write_to_with_name<W>(&self, name: &str, buff: &mut Vec<u8>) -> NbtResult<()> where W: NbtWrite {
        W::write_to_with_name(name, self, buff)
    }
    pub fn to_binary<W>(&self) -> NbtResult<Vec<u8>> where W: NbtWrite {
        W::to_bytes(self)
    }
    #[inline]
    pub fn as_i8(&self) -> NbtResult<i8> {
        match self {
            NbtValue::Byte(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(2, self.tag())),
        }
    }
    #[inline]
    pub fn as_i16(&self) -> NbtResult<i16> {
        match self {
            NbtValue::Short(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(2, self.tag())),
        }
    }
    #[inline]
    pub fn as_i32(&self) -> NbtResult<i32> {
        match self {
            NbtValue::Int(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(3, self.tag())),
        }
    }
    #[inline]
    pub fn as_i64(&self) -> NbtResult<i64> {
        match self {
            NbtValue::Long(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(4, self.tag())),
        }
    }
    #[inline]
    pub fn as_f32(&self) -> NbtResult<f32> {
        match self {
            NbtValue::Float(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(5_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_f64(&self) -> NbtResult<f64> {
        match self {
            NbtValue::Double(v) => Ok(*v),
            _ => Err(NbtError::IncorrectType(6_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i8_array(&self) -> NbtResult<Vec<i8>> {
        match self {
            NbtValue::ByteArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(7_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i32_array(&self) -> NbtResult<Vec<i32>> {
        match self {
            NbtValue::IntArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(11_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_i64_array(&self) -> NbtResult<Vec<i64>> {
        match self {
            NbtValue::LongArray(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(12_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_string(&self) -> NbtResult<String> {
        match self {
            NbtValue::String(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(8_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_list(&self) -> NbtResult<Vec<NbtValue>> {
        match self {
            NbtValue::List(v) => Ok(v.clone()),
            _ => Err(NbtError::IncorrectType(9_u8, self.tag())),
        }
    }
    #[inline]
    pub fn as_compound(&self) -> NbtResult<(Option<&String>, Vec<(String, NbtValue)>)> {
        match self {
            NbtValue::Compound(name, v) => Ok((name.as_ref(), v.clone())),
            _ => Err(NbtError::IncorrectType(10_u8, self.tag())),
        }
    }
    #[inline]
    pub fn is_i8(&self) -> bool { matches!(self, NbtValue::Byte(_)) }
    #[inline]
    pub fn is_i16(&self) -> bool { matches!(self, NbtValue::Short(_)) }
    #[inline]
    pub fn is_i32(&self) -> bool { matches!(self, NbtValue::Int(_)) }
    #[inline]
    pub fn is_i64(&self) -> bool { matches!(self, NbtValue::Long(_)) }
    #[inline]
    pub fn is_f32(&self) -> bool { matches!(self, NbtValue::Float(_)) }
    #[inline]
    pub fn is_f64(&self) -> bool { matches!(self, NbtValue::Double(_)) }
    #[inline]
    pub fn is_i8_array(&self) -> bool { matches!(self, NbtValue::ByteArray(_)) }
    #[inline]
    pub fn is_i32_array(&self) -> bool { matches!(self, NbtValue::IntArray(_)) }
    #[inline]
    pub fn is_i64_array(&self) -> bool { matches!(self, NbtValue::LongArray(_)) }
    #[inline]
    pub fn is_string(&self) -> bool { matches!(self, NbtValue::String(_)) }
    #[inline]
    pub fn is_list(&self) -> bool { matches!(self, NbtValue::List(_)) }
    #[inline]
    pub fn is_compound(&self) -> bool { matches!(self, NbtValue::Compound(_, _)) }
}
