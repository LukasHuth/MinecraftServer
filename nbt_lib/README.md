# NBT Lib

This is a simple library for reading and writing Minecraft NBT (Named Binary Tag) Data,
which is used for data exchange ans storage in Minecraft.
This crate was created in need of one when i was working on a Minecraft Server implementation.

# Disclaimer

This crate is heavily inspired by the work of [shenjack](https://github.com/shenjackyuanjie) and [Yue Fei](https://github.com/InfyniteHeap) in their crate [nbt-rust](https://github.com/shenjackyuanjie/nbt-rust).
Because this library draws from their work, it is not intended for standalone release.

## Data Sources
- [Minecraft Wiki](https://wiki.vg/NBT)
- [nbt-rust](https://github.com/shenjackyuanjie/nbt-rust)
  - big thanks to [shenjack](https://github.com/shenjackyuanjie) and [Yue Fei](https://github.com/InfyniteHeap) for their nice work on the mentioned crate
    their crate helped me a lot to make this one, because the wiki is a little bit missleading
    with their documentation some times. Please check out their amazing work. Sadly i couldn't just
    use their crate, because i need the possibility for adjustment to implement it into my server
    and the Chinese documentation makes this hard to archieve.

## Features

- reading compressed and uncompressed NBT data
- writing compressed and uncompressed NBT data (WIP)

## How to use

### Reading uncompressed Data
```rust
let mut file = File::open("name.nbt")?;
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;
let reader = nbt_lib::reader::NbtReader::new(buffer);
let nbt_data = nbt_lib::version::Java::from_reader(reader);
```

### Reading compressed Data
```rust
let mut file = File::open("name.nbt")?;
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;
let reader = nbt_lib::reader::NbtReader::from_compressed_data(buffer);
let nbt_data = nbt_lib::version::Java::from_reader(reader);
```
