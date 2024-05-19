//! This Module provides datastructs to manage a world

use std::{collections::HashMap, rc::Rc};

use nbt_lib::NbtValue;
pub mod chunks;
pub mod player;

/// A struct to manage a world
pub struct World {
    chunks: chunks::ChunkHolder,
    players: HashMap<u128, player::Player>,
    generator: Generator,
}
struct Generator {
    seed: i64,
}
impl Generator {
    pub fn get_height(&self, x: i64, z: i64) -> i64 {
        // f64::sin();
        let step = 2.0 * std::f64::consts::PI / 16.0 ;
        let offset = 60.0;
        let amplitude = 5.0;
        let x_comp = (-1.0 + (x as f64) * step).sin() * amplitude * -1.0;
        let z_comp = (-1.0 + (z as f64) * step).sin() * amplitude * -1.0;
        (offset + x_comp + z_comp) as i64
    }
}
impl World {
    fn generate_chunk(x: i64, z: i64) -> Result<Rc<NbtValue>, (i64, i64)> {
        Err((x, z))
    }
}
#[test]
fn test() {
    let step = 2.0 * std::f64::consts::PI / 16.0 ;
    let amplitude = 5.0;
    let offset = 60.0;
    for z in (0..4).step_by(4) {
        for x in 0..=16 {
            let x_comp = (-1.0 + (x as f64) * step).sin() * amplitude * -1.0;
            let z_comp = (-1.0 + (z as f64) * step).sin() * amplitude * -1.0;
            let comp = dbg!( f64::round(offset + x_comp + z_comp) );
        }
    }
    assert!(false)
}
