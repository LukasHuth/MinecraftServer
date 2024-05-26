use std::time::Instant;

use memory_management::world::chunks::ChunkHolder;

fn load_chunks_around(chunk_holder: &mut ChunkHolder, x: i64, z: i64, range: i64) {
    let mut counter = 0;
    for x in x-range..x+range {
        for z in z-range..z+range {
            // println!("({x},{z})");
            if let Ok(_) = chunk_holder.get(x, z) {
                counter += 1;
            }
        }
    }
    println!("loaded {counter} chunks");
}

#[tokio::main]
async fn main() {
    let mut chunk_holder: ChunkHolder = ChunkHolder::new("../nbt_lib/test_data/test_world/region");
    println!("{:?}", chunk_holder.get(17, 10));
    println!("starting to load all chunks");
    let loading_start_time = Instant::now();
    for x in 0..32u8 { for y in 0..32u8 { 
        let _ = chunk_holder.cache_region(x, y, 0, 0);
    }}
    for x in 0..32u8 { for y in 0..32u8 {
            let _ = chunk_holder.cache_region(x, y, -1, -1);
    }}
    println!("Loaded all chunks in {:?}", loading_start_time.elapsed());
    println!("uncompressed_chunks: {:?}", chunk_holder.count_uncompressed());
}
