# Minecraft Server
This is a project to learn, how to implement an documented packet (And possibly create a fully working Minecraft server).


## To-Do

- Move Packet handling out of lib
  - return packet type
  - create answer packet
    - eg. LegacyPongPacket (including toBytes())
  - make the client handling async
