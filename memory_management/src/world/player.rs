//! This module provides datastructs neccesarry for player data

use std::rc::Rc;
/// The location of a player where y is the height
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
/// An instance of a player
#[allow(missing_docs)]
pub struct Player {
    pub name: String,
    pub position: PlayerPosition,
    pub yaw: f32,
    pub pitch: f32,
    pub render_distance: f32,
    last_player_location_data: Option<PlayerLocationData>,
}
struct PlayerLocationData {
    last_position: PlayerPosition,
    last_yaw: f32,
    last_pitch: f32,
    last_visible_chunks: Rc<Vec<(i64, i64)>>,
}
impl Player {
    /// Returns all chunks that the player can see
    pub fn calculate_visible_chunks(&mut self) -> Rc<Vec<(i64, i64)>> {
        if let Some(ref last_player_location_data) = self.last_player_location_data {
            if last_player_location_data.last_yaw == self.yaw &&
                last_player_location_data.last_position == self.position {
                return last_player_location_data.last_visible_chunks.clone();
            }
        }
        /* Mindstructuring:
        *   yaw: 0 = North
        *   yaw: 180 = South
        *   yaw: 360 = 360%360 = 0 = North
        *   pitch: 0 = straigth ahead
        *   pitch: -90 = straight up
        *   pitch: 90 = straight down
        *   if pitch == 0 only send chunks in that direction
        *   if |pitch| == 90 send all to be sure, because depending in the height of the player
        *   they could be seeing more than expected
        */
        // This is just a solution for some thime, because it sends all possible chunks (way too
        // much data)
        let render_distance = self.render_distance as i64;
        let chunks: Vec<(i64, i64)> = (0..render_distance).map(|x|{
            let x = x + (self.position.x as i64 / 16);
            (0..render_distance).map(|z| {
                let z = z + (self.position.z as i64 / 16);
                (x,z)
            }).collect::<Vec<_>>()
        }).flatten().collect();
        let result = Rc::new(chunks);
        self.last_player_location_data = Some(PlayerLocationData {
            last_position: self.position,
            last_yaw: self.yaw,
            last_pitch: self.pitch,
            last_visible_chunks: result.clone(),
        });
        result
    }
}
