# Chip-8 Emulator in Rust


This project contains a Chip-8 Emulator written entirely in Rust. The goal of the project is to have an isolated Chip-8 Emulator that can easily integrate with various frontends and keyboard input events. There is an additional goal to write the emulator in such a way that uses all of the safe guarantees that Rust provides.

The project is currently a work in progress, and a few instructions are waiting to be implemented. Major things that are waiting to be completed are listed below.

Image of TicTac being played:
![image](screenshots/tic_tac.png)

Image of Brix being played:
![image](screenshots/brix.png)

Image of Space Invaders intro:
![image](screenshots/space_invaders_intro.png)

Image of Space Invaders being played:
![image](screenshots/space_invaders_play1.png)

## Usage

To use simply run:

```bash
$ cargo run --release -- $ROM_NAME
```

## Completed
- Chip-8 memory pattern
- Opcode interpretation
- Basic instructions are implemented (math/load registers/jumps)
- Basic understanding of how to work with tui-rs
- Keyboard input and emulation
- Emulate display and collision detection
- Draw Chip-8 display data to the terminal using tui-rs
- Logging

## TODO
- Add unit tests
- Add integration tests
- Rework the error system to use enumerated error messages
- Visualization of registers using tui-rs


