#![feature(box_into_inner)]
use std::{collections::HashMap, io::Read, path::Path, sync::Arc, time::{Duration, Instant, SystemTime}};

use flate2::bufread::{GzDecoder, ZlibDecoder};
use level_lib::anvil::region::CompressionScheme;
use memory_management::chunks::ChunkHolder;
use nbt_lib::{reader::NbtReader, traits::NbtRead, NbtValue};
use tokio::sync::Mutex;

async fn load_chunks_around(chunk_holder: &Arc<Mutex<ChunkHolder>>, x: i64, z: i64, range: i64) {
    let mut ch = chunk_holder.lock().await;
    println!("{}", ch.data.lock().unwrap().keys().len());
    let mut counter = 0;
    for x in x-range..x+range {
        for z in z-range..z+range {
            // println!("({x},{z})");
            if let Ok(_) = ch.get(x, z) {
                counter += 1;
            }
        }
    }
    println!("{}", ch.data.lock().unwrap().keys().len());
    println!("loaded {counter} chunks");
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut chunk_holder: ChunkHolder = ChunkHolder::new("../nbt_lib/test_data/test_world/region");
    // chunk_holder.lock().await.load_region(0, 0).unwrap();
    // chunk_holder.lock().await.load_region(-1, -1).unwrap();
    // println!("it took {:?} to load the whole data into the chunk holder", start.elapsed());
    /*
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, -8, 4, 16).await;
    println!("it took {:?} to load 16 chunks around (0|0)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, -8, 4, 16).await;
    println!("it took {:?} to load 16 chunks around (0|0)", chunk_load_start.elapsed());
    */
    /*
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, 0, 0, 16).await;
    println!("it took {:?} to load 16 chunks around (0|0)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, -24, 0, 16).await;
    println!("it took {:?} to load 16 chunks around (-24|0)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, 24, 0, 16).await;
    println!("it took {:?} to load 16 chunks around (24|0)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, 0, 24, 16).await;
    println!("it took {:?} to load 16 chunks around (0|24)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, 0, -24, 16).await;
    println!("it took {:?} to load 16 chunks around (0|-24)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, -24, -24, 16).await;
    println!("it took {:?} to load 16 chunks around (-24|-24)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, 24, -24, 16).await;
    println!("it took {:?} to load 16 chunks around (24|-24)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, -24, 24, 16).await;
    println!("it took {:?} to load 16 chunks around (-24|24)", chunk_load_start.elapsed());
    let chunk_load_start = Instant::now();
    load_chunks_around(&chunk_holder, 24, 24, 16).await;
    println!("it took {:?} to load 16 chunks around (24|24)", chunk_load_start.elapsed());
    */
    // let chunk_load_start = Instant::now();
    // let _chunk = chunk_holder.lock().await.get_from_region(1, 17, 0, 0).await.unwrap();
    // println!("it took {:?} to load a cached chunk", chunk_load_start.elapsed());
    println!("starting to load all chunks");
    let loading_start_time = Instant::now();
    for x in 0..32u8 { for y in 0..32u8 { 
        let _ = chunk_holder.cache_region(x, y, 0, 0);
    }}
    for x in 0..32u8 { for y in 0..32u8 {
            let _ = chunk_holder.cache_region(x, y, -1, -1);
    }}
    println!("Loaded all chunks in {:?}", loading_start_time.elapsed());
    println!("uncompressed_chunks: {}", chunk_holder.count_uncompressed());
}
