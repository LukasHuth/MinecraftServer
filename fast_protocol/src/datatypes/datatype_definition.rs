use nbt_lib::NbtValue;

pub mod important_enums;

// TODO replace fastnbt with own nbt value

pub struct EntityMetadata(entmet_lib::EntityMetadata);
pub struct Slot(slot_lib::Slot);
pub struct TextComponent(NbtValue);
pub struct NBT(NbtValue);
