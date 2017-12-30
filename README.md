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
- Running instructions
- Keyboard input and emulation
- Emulate display and collision detection
- Display data using sdl2
- Logging

## TODO
- Rework the error system
- Add unit tests
- Add integration tests

