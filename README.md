# t2tclient

### Wat iz dis???

This is a TUI client for the [t2tmud](https://t2tmud.org/) game.

What is a TUI? [Terminal User Interface](https://docs.rs/tui/0.16.0/tui/index.html)

What is a MUD? [Multi-User Dungeon](https://en.wikipedia.org/wiki/MUD)

Now that we've got that out of the way...

### Getting Started

You'll need [Rust](https://www.rust-lang.org/tools/install) installed.

Clone repo: `git clone https://github.com/Caryyon/t2tclient.git`

Inside the repo you can run `cargo run`

This will install all the dependencies as well as run the program.

### Goal

The goal here is to have a wrapper around the connection to the server with telnet.
Capture all the servers output and feed your own input back to the server.
The reason for doing this is we can capture common more visual output from the server into more visually represented boxes.
This allows us to have an easier experience moving around the world as well as viewing our inventory.



