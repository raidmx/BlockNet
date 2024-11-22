## BlockNet

Minecraft: Bedrock Edition Protocol and Networking Library in Rust. This library contains API & Framework for creating proxy, server or a client compatible with the Protocol Definitions from Mojang. This project uses tokio for multithreading for the networking part. The following components are included in this library:

1. Bedrock Protocol Library: Support for reading and writing of Minecraft Bedrock Edition Packets
2. RakNet: MCPE compatible UDP layer for reading and writing packets to a continuous stream providing TCP like architecture.
3. NBT: Support for Named Binary Tag parsing and serialising NBT data in worlds, player saves and over the network.
4. Basic client implementation: For purposes of creating of bots
5. Basic server implementation: For purposes of creating a MCPE server