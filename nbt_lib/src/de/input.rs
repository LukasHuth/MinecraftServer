use std::{borrow::Cow, io::Read, ops::Range};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{error::{Error, Result}, NbtTypeId};

mod private {
    pub trait Sealed {}
}
fn try_size(size: i32, multiplier: usize) -> Result<usize> {
    let size: usize = size
        .try_into()
        .map_err(|_| Error::Message("size was negative".to_string()))?;

    size.checked_mul(multiplier)
        .ok_or_else(|| Error::Message("size too large".to_string()))
}
pub enum Reference<'b, 'c, T> where T: ?Sized + 'static {
    Borrowed(&'b T),
    Copied(&'c T),
}
impl<'b, 'c> AsRef<[u8]> for Reference<'b, 'c, [u8]> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Reference::Borrowed(bs) => bs,
            Reference::Copied(bs) => bs,
        }
    }
}

pub trait Input<'de>: private::Sealed {
    fn consume_byte(&mut self) -> Result<u8>;
    fn ignore_str(&mut self) -> Result<()>;
    fn ignore_bytes(&mut self, size: usize) -> Result<()>;
    fn consume_tag(&mut self) -> Result<NbtTypeId> {
        let tag = self.consume_byte()?;
        NbtTypeId::try_from(tag).map_err(|_| Error::Message(format!("Invalid tag {:?}", tag)))
    }
    fn consume_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>>;
    fn consume_bytes<'s>(&'s mut self, n: usize, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>>;
    fn consume_i16(&mut self) -> Result<i16>;
    fn consume_i32(&mut self) -> Result<i32>;
    fn consume_i64(&mut self) -> Result<i64>;
    fn consume_f32(&mut self) -> Result<f32>;
    fn consume_f64(&mut self) -> Result<f64>;
    fn ignore_value(&mut self, tag: NbtTypeId) -> Result<()> {
        match tag {
            NbtTypeId::Byte => {
                self.consume_byte()?;
            }
            NbtTypeId::Short => {
                self.consume_i16()?;
            }
            NbtTypeId::Int => {
                self.consume_i32()?;
            }
            NbtTypeId::Long => {
                self.consume_i64()?;
            }
            NbtTypeId::Float => {
                self.consume_f32()?;
            }
            NbtTypeId::Double => {
                self.consume_f64()?;
            }
            NbtTypeId::String => {
                self.ignore_str()?;
            }
            NbtTypeId::ByteArray => {
                let size = self.consume_i32()? as usize;
                self.ignore_bytes(size)?;
            }
            NbtTypeId::IntArray => {
                let size = self.consume_i32()?;
                self.ignore_bytes(try_size(size, std::mem::size_of::<i32>())?)?;
            }
            NbtTypeId::LongArray => {
                let size = self.consume_i32()?;
                self.ignore_bytes(try_size(size, std::mem::size_of::<i64>())?)?;
            }
            NbtTypeId::Compound => {
                // Need to loop and ignore each value until we reach an end tag.

                // we need to enter the compound, then ignore it's value.
                loop {
                    let tag = self.consume_tag()?;
                    if tag == NbtTypeId::End {
                        break;
                    }

                    // consume the name.
                    self.ignore_str()?;
                    self.ignore_value(tag)?;
                }
            }
            NbtTypeId::List => {
                let element_tag = self.consume_tag()?;
                let size = self.consume_i32()?;
                for _ in 0..size {
                    self.ignore_value(element_tag)?;
                }
            }
            NbtTypeId::End => {
                // If we are trying to ignore a list of empty compounds, that
                // list might be indicated by a series of End tags. If this
                // occurs then we should end the Compound branch of this match
                // statement, where the end tag will be consumed. So we should
                // never reach here.
                //
                // TODO: Write an explicit test for ignored list of compound.
                unreachable!()
            }
        }

        Ok(())
    }
}

pub(crate) struct Slice<'de> {
    pub data: &'de [u8],
}
impl<'de> private::Sealed for Slice<'de> {}
impl<'de> Slice<'de> {
    fn consume(&mut self, r: Range<usize>) -> Result<&'de [u8]> {
        if r.end <= self.data.len() {
            let ret = &self.data[r.start..r.end];
            self.data = &self.data[r.end..];
            Ok(ret)
        } else {
            Err(Error::Eof)
        }
    }
}

impl<'de> Input<'de> for Slice<'de> {
    fn consume_byte(&mut self) -> Result<u8> {
        Ok(self.consume(0..1)?[0])
    }

    fn ignore_str(&mut self) -> Result<()> {
        let len = self.consume(0..2)?.read_u16::<BigEndian>()? as usize;
        self.consume(0..len).map(|_| ())
    }

    fn consume_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        let len = self.consume(0..2)?.read_u16::<BigEndian>()? as usize;
        let str = self.consume(0..len)?;
        let str = cesu8::from_java_cesu8(str).map_err(|_| Error::Message(format!("following data was supplied as string but isn't: '{str:?}'")))?;

        Ok(match str {
            Cow::Borrowed(str) => Reference::Borrowed(str),
            Cow::Owned(str) => {
                *scratch = str.into_bytes();
                // we just converted scratch into the bytes of a string, so it
                // definitely utf8.
                Reference::Copied(unsafe { std::str::from_utf8_unchecked(scratch) })
            }
        })
    }

    fn consume_bytes<'s>(
        &'s mut self,
        n: usize,
        _scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        let bs = self.consume(0..n)?;
        Ok(Reference::Borrowed(bs))
    }

    fn consume_i16(&mut self) -> Result<i16> {
        let mut bs = self.consume(0..std::mem::size_of::<i16>())?;
        Ok(bs.read_i16::<BigEndian>()?)
    }

    fn consume_i32(&mut self) -> Result<i32> {
        let mut bs = self.consume(0..std::mem::size_of::<i32>())?;
        Ok(bs.read_i32::<BigEndian>()?)
    }

    fn consume_i64(&mut self) -> Result<i64> {
        let mut bs = self.consume(0..std::mem::size_of::<i64>())?;
        Ok(bs.read_i64::<BigEndian>()?)
    }

    fn consume_f32(&mut self) -> Result<f32> {
        let mut bs = self.consume(0..std::mem::size_of::<f32>())?;
        Ok(bs.read_f32::<BigEndian>()?)
    }

    fn consume_f64(&mut self) -> Result<f64> {
        let mut bs = self.consume(0..std::mem::size_of::<f64>())?;
        Ok(bs.read_f64::<BigEndian>()?)
    }

    fn ignore_bytes(&mut self, size: usize) -> Result<()> {
        self.consume(0..size)?;
        Ok(())
    }
}

pub(crate) struct Reader<R: Read> {
    pub reader: R,
}

impl<R: Read> private::Sealed for Reader<R> {}

impl<'de, R: Read> Input<'de> for Reader<R> {
    fn consume_byte(&mut self) -> Result<u8> {
        Ok(self.reader.read_u8()?)
    }

    fn ignore_str(&mut self) -> Result<()> {
        let len = self.reader.read_u16::<BigEndian>()? as usize;
        let mut buf = vec![0; len]; // TODO: try a scratch space to reduce allocs?
        Ok(self.reader.read_exact(&mut buf)?)
    }

    fn consume_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        let len = self.reader.read_u16::<BigEndian>()? as usize;
        scratch.clear();
        scratch.resize(len, 0);
        self.reader.read_exact(scratch)?;

        let str = cesu8::from_java_cesu8(scratch).map_err(|_| Error::Message(format!("following data was supplied as string but isn't: '{scratch:?}'")))?;

        Ok(match str {
            Cow::Borrowed(_) => {
                Reference::Copied(unsafe { std::str::from_utf8_unchecked(scratch) })
            }
            Cow::Owned(s) => {
                *scratch = s.into_bytes();
                Reference::Copied(unsafe { std::str::from_utf8_unchecked(scratch) })
            }
        })
    }

    fn consume_bytes<'s>(
        &'s mut self,
        n: usize,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        scratch.clear();
        scratch.resize(n, 0);
        self.reader.read_exact(scratch.as_mut_slice())?;

        Ok(Reference::Copied(scratch.as_slice()))
    }

    fn consume_i16(&mut self) -> Result<i16> {
        Ok(self.reader.read_i16::<BigEndian>()?)
    }

    fn consume_i32(&mut self) -> Result<i32> {
        Ok(self.reader.read_i32::<BigEndian>()?)
    }

    fn consume_i64(&mut self) -> Result<i64> {
        Ok(self.reader.read_i64::<BigEndian>()?)
    }

    fn consume_f32(&mut self) -> Result<f32> {
        Ok(self.reader.read_f32::<BigEndian>()?)
    }

    fn consume_f64(&mut self) -> Result<f64> {
        Ok(self.reader.read_f64::<BigEndian>()?)
    }

    fn ignore_bytes(&mut self, size: usize) -> Result<()> {
        let mut buf = vec![0; size]; // TODO: try a scratch space to reduce allocs?
        self.reader.read_exact(&mut buf)?;
        Ok(())
    }
}
