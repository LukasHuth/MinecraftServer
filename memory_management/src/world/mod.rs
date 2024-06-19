//! This Module provides datastructs to manage a world

use std::{collections::HashMap, rc::Rc, time::SystemTime};

use level_lib::anvil::region::chunk::
    chunk_data::{ChunkData, GenerationStatus}
;

use self::chunks::ChunkHolder;
pub mod chunks;
pub mod player;

/// A struct to manage a world
pub struct World {
    chunks: chunks::ChunkHolder,
    _players: HashMap<u128, player::Player>,
    generator: Generator,
}
/// A struct representing a terrain generator.
///
/// The `Generator` struct uses a seed to generate terrain heights based on
/// a seed
pub struct Generator {
    seed: i64,
}
impl Generator {
    /// Calculates the height of the terrain at the specified coordinates `(x, z)`.
    ///
    /// This function uses sinusoidal functions to simulate terrain height.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate.
    /// * `z` - The z-coordinate.
    ///
    /// # Returns
    ///
    /// * The height of the terrain at the specified coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// use memory_management::world::Generator;
    ///
    /// let generator = Generator::new();
    /// let height = generator.get_height(10, 20);
    /// println!("Height at (10, 20): {}", height);
    /// ```
    pub fn get_height(&self, x: i64, z: i64) -> i64 {
        // f64::sin();
        let step = 2.0 * std::f64::consts::PI / 16.0;
        let offset = 60.0;
        let amplitude = 5.0;
        let x_comp = (-1.0 + (x as f64) * step).sin() * amplitude * -1.0;
        let z_comp = (-1.0 + (z as f64) * step).sin() * amplitude * -1.0;
        (offset + x_comp + z_comp) as i64
    }
    /// Creates a new `Generator` instance with a seed based on the current time.
    ///
    /// The seed is derived from the current time in nanoseconds since the UNIX epoch.
    ///
    /// # Returns
    ///
    /// * A new instance of `Generator`.
    ///
    /// # Example
    ///
    /// ```
    /// use memory_management::world::Generator;
    ///
    /// let generator = Generator::new();
    /// assert!(matches!(generator, Generator { .. }));
    /// ```
    pub fn new() -> Self {
        Self {
            seed: (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Using UNIX_EPOCH for the duration, so this should never fail")
                .as_nanos()
                & 0xFFFF) as i64,
        }
    }
    /// Returns the seed used by the generator.
    ///
    /// # Returns
    ///
    /// * The seed of the generator.
    ///
    /// # Example
    ///
    /// ```
    /// use memory_management::world::Generator;
    ///
    /// let generator = Generator::new();
    /// let seed = generator.get_seed();
    /// println!("Generator seed: {}", seed);
    /// ```
    pub fn get_seed(&self) -> i64 {
        self.seed
    }
}
impl World {
    /// Creates a new instance of `World`.
    ///
    /// This function initializes a new `World` with empty players, a new generator,
    /// and a `ChunkHolder` initialized with the provided region file location.
    ///
    /// # Arguments
    ///
    /// * `region_file_location` - A static string slice that specifies the location
    ///   of the region file to be used by the `ChunkHolder`.
    ///
    /// # Returns
    ///
    /// * A new instance of `World`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use memory_management::world::Generator;
    /// use memory_management::world::chunks::ChunkHolder;
    /// use memory_management::world::World;
    ///
    /// fn main() {
    ///     // Specify the region file location
    ///     let region_file_location = "../../../nbt_lib/test_data/test_world/region/";
    ///
    ///     // Create a new World instance
    ///     let world = World::new(region_file_location);
    ///
    ///     // Use the World instance
    ///     println!("New world created with region file: {}", region_file_location);
    /// }
    /// ```
    ///
    /// # See Also
    ///
    /// * [Generator]
    /// * [ChunkHolder]
    pub fn new(region_file_location: &'static str) -> Self {
        World {
            _players: HashMap::new(),
            generator: Generator::new(),
            chunks: ChunkHolder::new(region_file_location)
        }
    }
    fn generate_chunk(&self, x: i64, z: i64) -> Result<Rc<ChunkData>, (i64, i64)> {
        let heightmap: [[u16; 16]; 16] = (0..16)
            .map(|x_offset| {
                (0..16)
                    .map(|z_offset| {
                        (self
                            .generator
                            .get_height(x * 16 + x_offset, z * 16 + z_offset)
                            + 64) as u16
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let data = ChunkData::new(heightmap, "minecraft:stone", x, z, 0);
        Ok(Rc::new(data))
    }
    /// Retrieves a chunk at the specified coordinates `(x, z)`.
    ///
    /// This function first attempts to get an existing chunk from the internal storage.
    /// If the chunk is found and its generation status is `GenerationStatus::Full`, it is returned wrapped in `Ok`.
    ///
    /// If the chunk is not found or its generation status is not `Full`, a new chunk is generated for the specified coordinates.
    /// After generation, the chunk is inserted into the internal storage. If the insertion
    /// is successful, the generated chunk is returned wrapped in `Ok`.
    ///
    /// If both retrieving and generating the chunk fail, the function returns `Err`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the chunk.
    /// * `z` - The z-coordinate of the chunk.
    ///
    /// # Returns
    ///
    /// * `Ok(Rc<ChunkData>)` - If the chunk is found and has a `Full` generation status or is successfully generated.
    /// * `Err(())` - If the chunk cannot be retrieved or generated.
    ///
    /// # Errors
    ///
    /// This function returns `Err(())` if the chunk cannot be retrieved or generated, or if the retrieved chunk's generation status is not `Full`.
    ///
    /// # Example
    ///
    /// ```
    /// use memory_management::world::World;
    /// use level_lib::anvil::region::chunk::chunk_data::ChunkData;
    /// let mut world = World::new("../../../nbt_lib/test_data/test_world/region/");
    /// let chunk = world.get_chunk(1000, 1000).expect("This should not fail, because a chunks is getting
    /// generated, if the read chunk does not exist or is incomplete");
    /// assert!(matches!(chunk.as_ref(), &ChunkData { .. }));
    /// ```
    pub fn get_chunk(&mut self, x: i64, z: i64) -> Result<Rc<ChunkData>, ()> {
        let chunk = self.chunks.get(x, z);
        if let Ok(chunk) = chunk {
            if chunk.status == GenerationStatus::Full {
                return Ok(chunk);
            }
        }
        let chunk = dbg!(self.generate_chunk(x, z));
        if let Ok(chunk) = chunk {
            self.chunks.insert_generated_chunk(chunk.clone())?;
            return Ok(chunk);
        }
        Err(())
    }
}
