# Chip-8 Emulator in Rust

This project contains a Chip-8 Emulator written entirely in Rust. The goal of the project is to have an isolated Chip-8 Emulator that can easily integrate with various frontends and keyboard input events. There is an additional goal to write the emulator in such a way that uses all of the safe guarantees that Rust provides.

The project is currently a work in progress, and a few instructions are waiting to be implemented. Major things that are waiting to be completed are listed below.

# TODO
- Keyboard input and emulation
- Emulate display and collision detection
- Draw Chip-8 display data to the terminal using tui-rs
- Rewrite the registers in an enumerated and safe manner
- Rework the error system to use enumerated error messages
- Add logging
- Add visualization of registers using tui-rs

