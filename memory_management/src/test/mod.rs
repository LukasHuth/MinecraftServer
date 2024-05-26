use level_lib::anvil::region::chunk::{
    chunk_data::{ChunkData, GenerationStatus, Heightmaps, StructureDataList},
    section::{BlockData, ChunkSection},
};

extern crate test;
#[test]
#[deprecated]
fn test_chunk_loading() {
    macro_rules! generate_chunk_section {
        ($y:expr) => {
            ChunkSection {
                biome_data: None,
                biome_palette: vec!["minecraft:plains".to_string()],
                block_data: None,
                block_light: None,
                block_palette: vec![BlockData {
                    name: "minecraft:air".to_string(),
                    properties: None,
                }],
                sky_light: None,
                y: $y,
            }
        };
    }
    use crate::world::chunks::ChunkHolder;
    let mut chunk_holder: ChunkHolder = ChunkHolder::new("../nbt_lib/test_data/test_world/region");
    let chunk = chunk_holder.get(17, 10).unwrap();
    // println!("{:?}", chunk.sections);
    let expected: ChunkData = ChunkData {
        block_entities: Vec::new(),
        block_ticks: Vec::new(),
        fluid_ticks: Vec::new(),
        heightmaps: Heightmaps {
            motion_blocking: None,
            motion_blocking_no_leaves: None,
            world_surface: None,
            world_surface_wg: None,
            ocean_floor: None,
            ocean_floor_wg: None,
        },
        inhabited_time: 0,
        last_update: 109,
        sections: (-4..=19).map(|v| generate_chunk_section!(v)).collect(),
        status: GenerationStatus::StructureStarts,
        structure_data_list: StructureDataList {
            structure_references: Vec::new(),
            starts: Vec::new(),
        },
        x_pos: 17,
        z_pos: 10,
        y_pos: -4,
    };
    // println!("expected:\n{:?}", expected);
    assert_eq!(chunk.as_ref(), &expected);
}
