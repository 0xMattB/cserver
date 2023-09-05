# cserver

A simple practice console chat program written in Rust; server program, paired with the "cclient" program.

* 0.1.0: Preliminary design: Basic code structure, single-client communication, log-in routine; works with "cclient 0.1.0".
* 0.2.0: Updated main server routine to handle multiple clients and send broadcast messages; works with "cclient 0.2.0"
* 0.3.0: Cleaned up and refactored code, minor improvements; works with "cclient 0.3.0"
* 0.4.0: Neatening

Improvement ideas:
* ~~Text colorization~~
* Limit broadcast reception until log-in complete
* Password encryption
* Implementation of commands
* "!exit" command closes all clients and self
* Broadcast user log-in
