# SC08v2: Simple Computer (8-bits, version 2)

The SC08v2 is a super-simple, open-hardware 8bit computer built for retro hardware enthusiasts.

This repository contains various directories and files that help design, build and test both the computer's hardware and it's software. Those files and directories are the following:

| File/Dir | Explanation |
| --- | --- |
| **assembler/**            | converts assembly code to binary data. |
| **microcode/**            | files and scripts that help with designing and building the microcode for the computer. |
| **simulator/**            | a complete simulator to test software in. |
| **code/**                 | software for the CPU. |
| **code/so**               | complete OS. |
| **doc/**                  | documentation. |
| **kicad/**                | KiCAD hardware designs. |

## Developing software

Check the [programmer's manual](doc/programmers-manual.md) for a detailed guide on how to make software for the CPU.

## Running software in the CPU

To run software in the *simulated* CPU one has two follow two steps: compilation and execution. Let's run one of the test programs for example:

1. Compile the program by running the compiler: `python3 ./comp/compiler.py ./asm/test-programs/mult.asm`. This will result in a file named `result.bin` being created, which can be either flashed into the CPU's ROM or executed by the simulator. Note that the file has a size of exactly 64KB.
2. Run the simulator by entering the `sim` directory and then executing `cargo run ../result.bin | grep -E '^[0-9]+'`. In this case, enter a number (smaller than 255) and press enter. Repeat that step and the product of both will be outputted.

This shows that the simulator needs machine code and can not work with assembly code, since it is a one-to-one digital recreation of the real CPU.

## Contributing

Anything can be added to this repo, in whatever programming language you choose to make it, as long as it is a tool or some documentation for the SC08v2.

If you want to start somewhere, check the following section :)

### TODO

Right now, some very important thigs are missing from this repository. Aside from completing the already made modules, like the compiler, these things would be super cool to have:

- A program to automatically flash data to the CPU, both microcode and instructions.
- A diagram of the complete computer.
- A programmer's manual.
- A guide to using the CPU.
