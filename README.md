# Minecraft Server
This is a project to learn, how to implement an documented packet (And possibly create a fully working Minecraft server).

## To-Do
- [ ] add the missing Block states that are after [this](https://minecraft.fandom.com/wiki/Java_Edition_data_values#axis)
- [ ] using serde for the packets etc
  - help: [serde tutorial](https://serde.rs/impl-deserializer.html)
- [ ] implementing all the login packets
  - [x] packet implementation
  - [ ] packet usage (encrypted)
- [ ] implementint all the playing packets
  - [ ] implementing server handling (events redstone ticks etc.)
- [ ] test if inlining deref's helps performance

## Known Issues
- The Feature Flag Packet is currently brocken and is under investigation and will be fixed as fast as possible

## How to use

Follow these steps to set up and run the Rust Minecraft Server:

1. Clone the Repository and Navigate to the Project Directory
```bash
git clone git@github.com:LukasHuth/MinecraftServer.git
cd MinecraftServer
```

2. Configure the Server

- Open the `Config.toml` file.
- Customize the configurations according to your preferences, such as the port number, message of the day, maximum players, and offline mode.

3. Build the Server:

Compile using the Cargo executable

```bash
cargo build --release
```

4. Run the Server:
Start the server by executing the generated binary
```bash
./target/release/minecraft_server
```

## Configuration Config.toml

The `Config.toml` file holds essential configurations for the Rust Minecraft Server. Below is a breakdown of the key parameters:

- **port**: Specifies the port number for the server. Default is set to `25565`.
- **motd**: Stands for "Message of the Day," providing a brief message displayed to players on the serverlist. Default setting: `"A Cool Rust Minecraft Server"`.
- **max_players**: Defines the maximum number of players allowed on the server simultaneously. Default is `100`.
- **offline_mode**: Determines whether the server operates in offline mode, which allows players with unverified Minecraft accounts to join. Presently set to `false`.

Ensure to adjust these configurations according to your server requirements before deployment.

## Disclaimer

Shoutout to [shenjack](https://github.com/shenjackyuanjie) and [Yue Fei](https://github.com/InfyniteHeap) with their work on the crate [nbt-rust](https://github.com/shenjackyuanjie/nbt-rust).
Their crate helped me out a lot to implement a fine tuned working NBT library.
More Informations [here](./nbt_lib/README.md)

