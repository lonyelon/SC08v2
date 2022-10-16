# SSC08v02: Sergio's Super Simple Computer (8-bits, version 2)

The SSC08v2 is a super-simple, open-hardware 8bit computer built for retro hardware enthusiasts.

This repository contains various directories and files that help design, build and test both the computer's hardware and it's software. Those files and directories are the following:

<img src="doc/arch.svg"></img>

| File/Dir | Explanation |
| --- | --- |
| **compiler.py** | converts assembly code to binary data. |
| **micro/**      | files and scripts that help with designing and building the microcode for the computer. |
| **sim/**        | a complete simulator to test software in. |
| **asm/**        | software to be built for the CPU. |
| **asm/so**      | Complete OS. |

## Contributing

Anything can be added to this repo, in whatever programming language you choose to make it, as long as it is a tool or some documentation for the SSC08v02.

If you want to start somewhere, check the following section :)

### TODO

Right now, some very important thigs are missing from this repository. Aside from completing the already made modules, like the compiler, these things would be super cool to have:
- A program to automatically flash data to the CPU, both microcode and instructions.
- A diagram of the complete computer.
- A programmer's manual.
- A guide to using the CPU.
