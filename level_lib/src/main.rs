pub fn get_height(x: i64, z: i64) -> i64 {
    // f64::sin();
    let step = 2.0 * std::f64::consts::PI / 16.0 ;
    let offset = 60.0;
    let amplitude = 5.0;
    let x_comp = (-1.0 + (x as f64) * step).sin() * amplitude * -1.0;
    let z_comp = (-1.0 + (z as f64) * step).sin() * amplitude * -1.0;
    (offset + x_comp + z_comp) as i64 + 64
}
fn main() {
    let heightmap: [[u16; 16]; 16] = (0..16).map(|x| (0..16).map(|y| get_height(x, y) as u16).collect::<Vec<_>>().try_into().unwrap()).collect::<Vec<_>>().try_into().unwrap();
    let test = level_lib::anvil::region::chunk::chunk_data::ChunkData::create_pregen(heightmap, "");
}
