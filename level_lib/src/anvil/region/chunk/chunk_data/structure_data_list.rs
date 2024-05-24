use std::collections::HashMap;

use nbt_lib::{traits::{AsNbtValue, FromNbtValue}, NbtValue};

use nbt_lib::{create_compound_map, unwrap_to_empty};

use super::ChunkDataHolder;


/// Struct to hold all structure data of a Chunk
pub struct StructureDataList {
    /// A list of the structure coordinates withing one chunk
    ///
    /// # Note
    ///
    /// In NBT it's represented as an i64
    /// (x << 32) | y
    pub structure_references: Vec<(String, Vec<[u32; 2]>)>,
    ///  Structures that are yet to be generated, stored by general type. Some parts of the
    ///  structures may have already been generated. Completely generated structures are removed by
    ///  setting their id to "INVALID" and removing all other tags.
    pub starts: Vec<(String, ChunkDataHolder)>,
}
impl FromNbtValue for StructureDataList {
    fn from_nbt_value(value: NbtValue) -> Result<Self, ()> where Self: Sized {
        let (_, data) = unwrap_to_empty!(Some(value), compound);
        let structure_references: Vec<(String, Vec<[u32;2]>)> = unwrap_to_empty!(data.get("structure_references"), compound).1
            .into_iter()
            .map(|(s, d)|{
                combine_result(s, d.as_i64_array().map_err(|_|()))
            }).collect::<Result<Vec<(String, Vec<i64>)>, ()>>()?
            .into_iter()
            .map(|(s, data)|(s, data.into_iter().map(|data|[(data >> 32) as u32, (data & 0xFFFFFFFF) as u32]).collect()))
            .collect();
        let starts: Vec<(String, ChunkDataHolder)> = unwrap_to_empty!(data.get("starts"), compound).1.into_iter()
            .map(|(s, data)|{
                let cdh = ChunkDataHolder::from(s.clone(), data);
                match cdh {
                    Ok(data) => Ok((s, data)),
                    Err(()) => Err(()),
                }
            }).collect::<Result<Vec<_>, ()>>()?;
        Ok(Self { structure_references, starts })
    }
}
#[inline]
fn combine_result(s: String, data: Result<Vec<i64>, ()>) -> Result<(String, Vec<i64>), ()> {
    match data {
        Ok(data) => Ok((s, data)),
        Err(()) => Err(()),
    }
}
impl StructureDataList {
    /// creates a new structure data list
    pub fn new() -> Self {
        Self {
            structure_references: Vec::new(),
            starts: Vec::new(),
        }
    }
}
impl AsNbtValue for StructureDataList {
    fn as_nbt_value(&self) -> Result<NbtValue, ()> {
        use nbt_lib::NbtValue::*;
        use std::string::String as Str;
        let mut references_hash_map = HashMap::new();
        for struct_ref in self.structure_references.iter() {
            references_hash_map.insert(struct_ref.0.to_owned(), LongArray(struct_ref.1.iter().map(|e| ((e[0] as i64) << 32) | e[1] as i64).collect()));
        }
        Ok(Compound(None, create_compound_map!(
            References: Compound(None, references_hash_map),
            starts: Compound(None, self.starts.iter().map(|(s, d)| d.as_nbt_value().map(|v| (s.to_owned(), v))).collect::<Result<HashMap<Str, NbtValue>, ()>>()?)
        )))
    }
}
