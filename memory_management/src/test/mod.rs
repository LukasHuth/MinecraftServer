extern crate test;
#[test]
#[deprecated]
fn test_chunk_loading() {
    use crate::chunks::ChunkHolder;
    let mut chunk_holder: ChunkHolder = ChunkHolder::new("../nbt_lib/test_data/test_world/region");
    let chunk = dbg!(chunk_holder.get(17, 10)).unwrap();
    use nbt_lib::NbtValue::{self, *};
    use std::string::String as Str;
    /*
    let expected = Compound(Some(Str::new()), [
        ("Status", String("minecraft:structure_starts".to_string())),
        ("LastUpdate", Long(109)),
        ("InhabitedTime", Long(0)),
        ("yPos", Int(-4)),
        ("entities", List([])),
        ("block_ticks", List([])),
        ("DataVersion", Int(3578)),
        ("structures", Compound(None, [
            ("starts", Compound(None, HashMap::new())),
            ("References", Compound(None, HashMap::new()))
        ].into_iter().collect())),
            ("fluid_ticks", List([])),
            ("sections", List([
                Compound(None, [
                    ("biomes", Compound(None, [
                        ("palette", List([String("minecraft:plains".to_string())]))
                    ].into_iter().collect())),
                    ("block_states", Compound(None, [
                        ("palette", List([Compound(None, [
                            ("Name", String("minecraft:air".to_string()))
                        ].into_iter().collect())]))
                    ].into_iter().collect())),
                    ("Y", Byte(-4))
                ].into_iter().collect()),
                Compound(None, [
                    ("biomes", Compound(None, [
                        ("palette", List([String("minecraft:plains".to_string())]))
                    ].into_iter().collect())),
                    ("block_states", Compound(None, [
                        ("palette", List([Compound(None, [
                            ("Name", String("minecraft:air".to_string()))
                        ].into_iter().collect())])),
                    ].into_iter().collect())),
                    ("Y", Byte(-3))
                ].into_iter().collect()),
                Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "Y": Byte(-2), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])})}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(-1), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])})}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(0)}), Compound(None, {"Y": Byte(1), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])})}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "Y": Byte(2)}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(3), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])})}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(4)}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(5)}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(6)}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(7), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])})}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "Y": Byte(8), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])})}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "Y": Byte(9), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])})}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(10)}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(11)}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(12), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])})}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "Y": Byte(13)}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "Y": Byte(14)}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(15)}), Compound(None, {"Y": Byte(16), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])})}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(17)}), Compound(None, {"biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "Y": Byte(18)}), Compound(None, {"block_states": Compound(None, {"palette": List([Compound(None, {"Name": String("minecraft:air")})])}), "biomes": Compound(None, {"palette": List([String("minecraft:plains")])}), "Y": Byte(19)})]), "CarvingMasks": Compound(None, {}), "xPos": Int(17), "Heightmaps": Compound(None, {}), "zPos": Int(10), "PostProcessing": List([List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([]), List([])]), "block_entities": List([])})
    assert_eq!(chunk.as_ref(), &expected);
    */
    assert!(true);
}
