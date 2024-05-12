use std::{collections::{HashMap, HashSet}, fmt::Display, io::Read as _, path::Path, rc::Rc, sync::Mutex, time::{Duration, Instant, SystemTime}};

use flate2::read::{GzDecoder, ZlibDecoder, ZlibEncoder};
use level_lib::anvil::region::{CompressionScheme, LocationAndTimestampTable, RegionData};
use nbt_lib::{reader::NbtReader, traits::NbtRead as _, NbtValue};
use std::fmt::Debug;

#[derive(Clone)]
pub enum ChunkDataHolder {
    Raw(CompressionScheme, Vec<u8>),
    Data(Box<NbtValue>, Vec<u8>),
}
impl Debug for ChunkDataHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
impl Display for ChunkDataHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(_, _) => f.write_str("Raw chunk data"),
            Self::Data(_, _) => f.write_str("Interpreted chunk data")
        }
    }
}

impl ChunkDataHolder {
    pub fn compress(&mut self) -> Result<(), ()> {
        match self {
            ChunkDataHolder::Data(_, data) => {
                let mut compressed_data = ZlibEncoder::new(data.as_slice(), flate2::Compression::best());
                let mut data = Vec::new();
                compressed_data.read_to_end(&mut data).map_err(|_|())?;
                *self = Self::Raw(CompressionScheme::Zlib, data);
            },
            ChunkDataHolder::Raw(compression_data, data) => {
                if *compression_data != CompressionScheme::None { return Ok(()); }
                let mut compressed_data = ZlibEncoder::new(data.as_slice(), flate2::Compression::best());
                let mut data = Vec::new();
                compressed_data.read_to_end(&mut data).map_err(|_|())?;
                *self = Self::Raw(CompressionScheme::Zlib, data);
            }
        }
        Ok(())
    }
}

impl ChunkDataHolder {
    /// Returns `true` if the chunk data holder is [`Data`].
    ///
    /// [`Data`]: ChunkDataHolder::Data
    #[must_use]
    fn is_data_included(data: &&(Rc<Self>, SystemTime)) -> bool {
        matches!(*data.0, Self::Data(..))
    }
}
// #[derive(Clone)]
pub struct ChunkHolder {
    pub data: Mutex<HashMap<(i64, i64), (Rc<ChunkDataHolder>, SystemTime)>>,
    loaded_region: HashSet<(i64, i64)>,
    /// A list, when a specific chunk was last accessed
    last_accessed: HashMap<(i64, i64), Instant>,
    last_accessed_region: HashMap<(i64, i64), Instant>,
    last_save_and_update: HashMap<(i64, i64), (SystemTime, bool)>,
    region_file_location: &'static str,
}
impl ChunkHolder {
    pub fn count_uncompressed(&self) -> usize {
        self.data.lock().unwrap().values().filter(ChunkDataHolder::is_data_included).count()
    }
    pub fn new(region_file_location: &'static str) -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
            loaded_region: HashSet::new(),
            last_save_and_update: HashMap::new(),
            last_accessed: HashMap::new(),
            last_accessed_region: HashMap::new(),
            region_file_location
        }
    }
    #[inline]
    pub fn cache_region(&mut self, x: u8, y: u8, r_x: i64, r_y: i64) -> Result<Box<NbtValue>, (i64, i64)> {
        self.get(x as i64 + r_x * 32, y as i64 + r_y * 32)
    }
    #[inline]
    pub fn get_from_region(&mut self, x: u8, y: u8, r_x: i64, r_y: i64) -> Result<Box<NbtValue>, (i64, i64)> {
        self.get(x as i64 + r_x * 32, y as i64 + r_y * 32)
    }
    pub fn load_region(&mut self, r_x: i64, r_y: i64) -> Result<(), ()> {
        if self.loaded_region.contains(&(r_x, r_y)) { return Ok(()) }
        // let data = include_bytes!("../../nbt_lib/test_data/test_world/region/r.0.0.mca");
        let path_str = format!("{}/r.{}.{}.mca", self.region_file_location, r_x, r_y);
        let path = Path::new(path_str.as_str());
        let wraped_file_pointer = std::fs::File::open(path);
        if let Err(_) = &wraped_file_pointer { return dbg!(Err(())) }
        let mut file_pointer = wraped_file_pointer.unwrap();
        let mut latt: [u8;4096*2] = [0u8;4096*2];
        file_pointer.read_exact(&mut latt).map_err(|_|())?;
        let latt: LocationAndTimestampTable = LocationAndTimestampTable::from_bytes(latt);
        let mut other_data = Vec::new();
        file_pointer.read_to_end(&mut other_data).map_err(|_|())?;
        for x in 0..32 {
            // let row_load_start = Instant::now();
            for y in 0..32 {
                // let index = (x + y * 32) * 4;
                let location_data = latt.get(x, y);
                if location_data.offset == [0,0,0] && location_data.size == 0 { continue; }
                let offset = location_data.get_offset() as usize;
                let offset = offset * 4096;
                let start_index = offset-4096*2;
                // let size = location_data[3] as usize * 4096;
                let chunk_size = u32::from_be_bytes(other_data[start_index..][..4].try_into().unwrap()) as usize;
                let compression = match other_data[start_index+4] {
                    1 => CompressionScheme::Gzip,
                    2 => CompressionScheme::Zlib,
                    _ => unimplemented!(),
                };
                let chunk_data = &other_data[start_index+5..][..chunk_size];
                self.insert_from_region(x as u8, y as u8, r_x, r_y, compression, chunk_data.to_vec(), latt.get_timestamp_time(x, y)).unwrap();
            }
            // println!("chunks at x:{x} loaded in {:?}", row_load_start.elapsed());
        }
        self.loaded_region.insert((r_x, r_y));
        Ok(())
    }
    pub fn get<'a>(&'a mut self, x: i64, y: i64) -> Result<Box<NbtValue>, (i64, i64)> {
        let started_loading = Instant::now();
        static HUNDRED_MS: Duration = Duration::from_millis(100);
        if !self.data.lock().unwrap().contains_key(&(x, y)) { 
            // println!("{:?}", self.data.keys());
            if !self.loaded_region.contains(&(x, y)) {
                self.load_region(x / 32, y / 32).map_err(|_|(x, y))?;
            } else {
                return Err((x, y));
            }
        }
        if self.data.lock().unwrap().contains_key(&(x, y)) { self.set_accessed(x, y) }
        let generated_data: ((i64, i64), (Rc<ChunkDataHolder>, SystemTime));
        self.set_accessed(x, y);
        if let Some((chunk_data, chunk_data_timestamp)) = self.data.lock().unwrap().get(&(x, y)) {
            match chunk_data.as_ref() {
                ChunkDataHolder::Data(data, ..) => return Ok(data.clone()),
                ChunkDataHolder::Raw(compression, raw_data) => {
                    let data_buffer;
                    let chunk_data = match compression {
                        CompressionScheme::Zlib => {
                            let decoder = ZlibDecoder::new(raw_data.as_slice());
                            data_buffer = decoder.bytes().into_iter().map(|e|e.unwrap()).collect::<Vec<_>>();
                            &data_buffer
                        }
                        CompressionScheme::Gzip => {
                            let decoder = GzDecoder::new(raw_data.as_slice());
                            data_buffer = decoder.bytes().into_iter().map(|e|e.unwrap()).collect::<Vec<_>>();
                            &data_buffer
                        }
                        CompressionScheme::None => {
                            &raw_data
                        }
                        _ => unimplemented!()
                    };
                    let uncompressed_chunk_data = chunk_data;
                    let reader = NbtReader::new(uncompressed_chunk_data.to_owned());
                    let result = nbt_lib::version::Java::from_reader(reader);
                    if let Ok(chunk_data) = result {
                        if started_loading.elapsed().gt(&HUNDRED_MS) {
                            println!("it took {:?} to load ({x:3}{y:3})", started_loading.elapsed());
                        }
                        let chunk_data = Box::new(chunk_data);
                        generated_data = ((x, y), (Rc::new(ChunkDataHolder::Data(chunk_data, uncompressed_chunk_data.to_owned())), *chunk_data_timestamp));
                    } else { return Err((x, y)); }
                }
            }
        } else {
            return Err((x, y));
        }
        self.data.lock().unwrap().insert(generated_data.0, generated_data.1);
        self.get(x, y)
    }
    fn set_accessed(&mut self, x: i64, y: i64) {
        self.last_accessed.insert((x, y), Instant::now());
        self.last_accessed_region.insert((x / 32, y / 32), Instant::now());
    }
    #[inline]
    fn insert_from_region(&mut self, x: u8, y: u8, r_x: i64, r_y: i64, compression: CompressionScheme, data: Vec<u8>, timestamp: SystemTime) -> Result<(), ()> {
        self.insert(x as i64 + r_x * 32, y as i64 + r_y * 32, compression, data, timestamp)
    }
    fn insert(&mut self, x: i64, y: i64, compression: CompressionScheme, data: Vec<u8>, timestamp: SystemTime) -> Result<(), ()> {
        if self.data.lock().unwrap().contains_key(&(x, y)) { return Err(()); }
        self.data.lock().unwrap().insert((x, y), (Rc::new(ChunkDataHolder::Raw(compression, data)), timestamp));
        Ok(())
    }
    fn unload_region(&mut self, r_x: i64, r_y: i64) -> Result<(), ()>{
        /*
        let mut test = self.data.lock().unwrap();
        let mut chunks: Vec<((i64, i64), &mut (Rc<ChunkDataHolder>, SystemTime))> = test.iter_mut()
        .filter(|((x, y), (_,_))|(x / 32) == r_x && (y / 32) == r_y)
        .map(|((x, y), data)|((x & 31, y & 31), data))
        .collect();
        for (_, (chunk, _)) in chunks.iter_mut() {
        chunk.compress()?;
        }
        let region_data: RegionData = RegionData::new();
        */
        Ok(())
    }
    pub async fn update(&mut self) {
        const UNLOAD_TIME: Duration = Duration::from_secs(10 * 60);
        const DISCARD_TIME: Duration = Duration::from_secs(1 * 60 * 60);
        self.data.lock().unwrap().iter_mut().for_each(|(cords, e)| {
            if let ChunkDataHolder::Data(_, raw_data) = e.0.as_ref() {
                if self.last_accessed.get(cords).unwrap_or(&Instant::now()).elapsed().lt(&UNLOAD_TIME) { return; }
                e.0 = Rc::new(ChunkDataHolder::Raw(CompressionScheme::None, raw_data.to_owned()));
            }
        });
        // TODO: Change the unwrap to some sort of error handling
        self.last_accessed_region.clone().iter().filter(|(_, i)|i.elapsed().ge(&DISCARD_TIME)).for_each(|(coords, _)|self.unload_region(coords.0, coords.1).unwrap());
        if self.data.lock().unwrap().values().count() > LOADED_CHUNKS_LIMIT {
            let self_clone = self.last_accessed_region.clone();
            let mut lar: Vec<(&(i64, i64), &Instant)> = self_clone.iter().collect();
            lar.sort_by(|(_, i0),(_, i1)|i0.cmp(i1));
            while self.data.lock().unwrap().keys().count() > LOADED_CHUNKS_LIMIT {
                let mut lar_iter = lar.iter();
                loop {
                    let oldest_access = lar_iter.next().unwrap();
                    if let Ok(()) = self.unload_region(oldest_access.0.0, oldest_access.0.1) {
                        break;
                    }
                }
            }
            println!("oldest_access: {:?}", lar.first().unwrap().1.elapsed());
        }
    }
}
const LOADED_CHUNKS_LIMIT: usize = 100_000;
impl std::fmt::Display for ChunkHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ChunkHolder {\n")?;
        let indent = 2;
        for (key, value) in self.data.lock().unwrap().iter() {
            f.write_fmt(format_args!("{}({:3}|{:3}): ", " ".repeat(indent), key.0, key.1))?;
            match value.0.as_ref() {
                ChunkDataHolder::Data(..) => f.write_str("NBT Data")?,
                ChunkDataHolder::Raw(..) => f.write_str("raw NBT Data")?,
            }
            f.write_str("\n")?;
        }
        f.write_str("}")?;
        Ok(())
    }
}
