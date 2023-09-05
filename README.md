# cserver

A simple practice console chat program written in Rust; server program, paired with the "cclient" program.

## Description

Run this program before running the "cclient" program. It expects an argument of the filename containing the username and passwords. If the indicated file is not found, a new file will be created when the first new user is created.

The IP/Port the server is listening on is currently hard-coded as "192.168.1.121:4915".

## Getting Started

### Dependencies

There are currently no depencies for this project.

### Installing

Currently, downloading the source code and compiling locally is the only way to run this program.
```
// with Rust installed:
cargo new cserver
cd cserver
// copy source code into this directory (the "keys.txt" file is a sample username/password file)
// compile the program
cargo build
```

### Executing program

* The program can be executed via the Rust environment:
```
cargo run -- keys.txt
```
* The program can also be executed via the .exe file:
```
cserver.exe keys.txt
```

## Authors

0xMattB

## Version History

* 0.4.0
    * Neatened up code
* 0.3.0
    * Cleaned up and refactored code
    * Minor improvements
    * Works with "cclient 0.3.0"
* 0.2.0
    * Updated main server routine to handle multiple clients and send broadcast messages
    * Works with "cclient 0.2.0"
* 0.1.0
    * Preliminary design
    * Basic code structure
    * Single-client communication
    * Log-in routine
    * Works with "cclient 0.1.0"

## Improvement Ideas

* ~~Text colorization~~
* Limit broadcast reception until log-in complete
* Password encryption
* Implementation of commands
* "!exit" command closes all clients and self
* Broadcast user log-in

## License

T.B.D.
