# The programmer's manual

This is the manual for making software for the ssc08v02.

## The basics

We use the assembler located at `comp/assembler.py` to convert assembly code into a ROM binary file. The assembly code we use is a custom one made with simplicity in mind, so it should not be really hard to get used to it.

The instructions in the assembly language are made of two elements, the instruction itself and the modifier. For example `LDD.inp` means "load data into the data registry from the input", while `LDD.ram 0x00` means "load data into the data registry from ram at position 0x00". As you can see, arguments need to be passed to instructions sometimes, like the `0x00` before. We will explore the most important instructions and modifiers later, so don't worry too much about this.

To run your software, stored in a `result.bin` file generated after assembling, we can use the program located in `sim/`, which is a full simulator of the CPU written in rust. To un it, use `cargo run <file>`, where the file is out `result.bin`.

## Our first program

Let's build a really basic program for the CPU. In this case, we want to get a number from the user, add one to it and then print it. That program is four bytes in size, and looks like this:

```asm
        LDD.inp
        ADD.num 1
        STD.out
```

We will ignore the tabs at the beggining of each line for noew, we will revisit them later when looking into the JUM instruction.

Now, let's make a more complicated program. In this case, we want the user to give two numbers and then add them together. Since we now need two numbers, and we only have one data registry, we need to store the first number to ram to prevent the second one from overwriting it. Then, we add the second number, the one we already have in the data registry, with the first one, placed in the ram at position 0 (0x00).

```asm
        LDD.inp
        STD.ram 0x00
        LDD.inp
        ADD.ram 0x00
        STD.out
```

Manually managing ram positions can be a pain, so the assembler allows us to assign names to ram positions so that we don't lose track of them:

```asm
#var    first   0x00
        LDD.inp
        STD.ram first
        LDD.inp
        ADD.ram first
        STD.out
```

And, as we can see, the result is much cleaner. 

### Comments and blank lines

Comments can be added at any place of the program to explain what the software is doing. The following code is the program we already made, but commented so that someone reading it knows what we are doing:

```asm
#var    first   0x00

        ;; Get the first number.
        LDD.inp
        STD.ram first
        
        ;; Get the second number.
        LDD.inp
        
        ;; Add them and show the result.
        ADD.ram first
        STD.out

;; Comments can go here too btw, it really doesn't matter.
```

Blank lines can also be really helpful to make the code more readable, so use them freely :).

### The JUM instruction

Another very important instruction is the JUM instruction. This one allows jumping (surprise?) to a position in ROM. But things aren't that easy. The JUM instruction ONLY jumps if the data registry has a value different from 0. So if we want to jump unconditionally, we must run something like:

```asm
        LDD.num 0xff
        JUM.num 0x1234
```

Isn't that "0x1234" weird? This is an 8bit CPU, but that number is 16 bits in size. That's because ROM addresses are 16bits in size to allow 64KB of storage. Since the bus (or anything in the CPU, really) can't handle more than 8 bits of data at a given time, the number is split into two and read sequentially. That results on the ROM looking something like this:

```asm
0x0003 JUM.num
0x0004 0x12
0x0005 0x34
```

Let's use the JUM instruction to make an infinite loop:

```asm
        LDD.num 0xff
        JUM.num 0x0003
```

This is some really awful code, mainly because it is impossible to look at it and know what it is doing. We see that it is loading 0xff to the data registry (so we can jump) and then jumping to rom 0x0003. What is that rom position? we may not know, and that is why it is awful. If we count the rom positions, we see that the 0th one is reserved (we won't explain why now), the first one is used for the `LDD.num` instruction, the second one for the number to be loaded (`0xff`), the third one for the jump instruction, the... Eureka! We are jumping to the third position in the ROM, which is the `JUM.num` instruction, so this is an infinite loop that jumps to itself!

See how hard reading that code is? That's why the assembler allows for naming the positions in the ROM, because in more complicated programs, counting manually to the position we want to jump is impossible, and that position may change in the future if we add instruction in between! Using position names, the program just becomes:

```asm
        LDD.num 0xff
halt    JUM.num :halt
```

Which we can easily understand. This is the reason we have to always include tabs or spaces before the instructions, in case we want to put a name to them.

### Comparisons

JUM instructions only become useful once comparisons are understood. The CPU includes two kinds of comparisons, equality (EQU) and greater (GRE), but combining both we can get many others.

The comparison instructions take whatever is in the data registry, compare it to a value we give them, and then store the result back into the data registry. For example, the following program asks the user for a number, and if the number is `33`, then it jumps to another place in the code:

```asm
        LDD.inp
        EQU.num 33
        JUM.num :somewhere
```

Let's see what is happening here... first, the user is being asked for a number, which is getting stored in the data registry. Then, the number is compared to 33, and if it is equal, `0xff` is written to the data registry and so the JUM instruction is executed. If the number is not 33, then `0x00` is stored into the dta registry and the jump instruction is passed.

So that is great, we can do a `== 33` comparison and a `> 33` one, but how about something like `<= 33`? For that one, we need to combine the previous two:

```asm
        ;; Get the number from the user and store it.
        LDD.inp
        STD.ram 0x00
        
        ;; Check if it is 33, if it is, jump to "true".
        EQU.num 33
        JUM.num :true
        
        ;; Check if the number is greater than 33, if it is, jump to "false".
        LDD.ram 0x00
        GRE.num 33
        JUM.num :false
        
        ;; This gets executed if the first jump executed or if the last one
        ;; didn't.
:true   LDD.num 1
        STD.out
        LDD.num 0xff
        JUM.num :halt

        ;; This gets executed if the first jump didn't get executed but the last
        ;; one did.
:false  LDD.num 0
        STD.out
        LDD.num 0xff
        JUM.num :halt
        
halt    JUM.num :halt
```

As always, writing this is a pain, but in this case there is no solution for it. Work is being done in a macro system to define a new instruction like `LES.num 33` that gets transformed into the code above at compile-time, but this won't be available soon.

### Pointers

Pointers are a hard-to-understand tool, but a really powerfull one. We will assume going forward the the reader is familiar with how pointers work.

Sometimes the value we have in the data registry is not intended to be a value by itself, but a pointer to a position in ram. For example, in the following program we see how we can access the 16th position in ram by loading the number into the data registry and then resolving the pointer:

```asm
        LDD.num 0x10 ;; Load 16 into DTA.
        LDD.ptr      ;; Load the 16th position in ram to DTA.
```

Maybe the value we load is by itself a pointer to another place, which can be resolved with ease again:

```asm
        LDD.num 0x10 ;; Load 16 into DTA.
        LDD.ptr      ;; Load the 16th position in ram to DTA.
        LDD.ptr      ;; Load the XXth position in ram to DTA.
```

To save instructions, instead of loading data into DTA, we may just use the `prr` (pointer-to-ram) modifier. The following program does the same as the above one, but it saves one instruction:

```asm
        LDD.prr 0x10 ;; Load RAM at position 0x10, use that as a pointer and resolve it.
        LDD.ptr      ;; Same as before.
```

We could be confused on why to use `ptr` when `prr` exists, but there are cases where it's needed. The one above is one good example of it, but many others can be shown. The next program would be more complicated if we only used `prr` and not `ptr`:

```asm
        LDD.ram 0x00
        ADD.num 2
        LDD.ptr
        STD.out
```

For a last example, the next program print all the contents in the RAM except for the first position, used for the counter. We leave the explanation as homework for the reader.

```asm
#var    counter 0x00

        ;; Load and increment the counter.
start   LDD.ram counter
        ADD.num 1
        STD.ram counter
        
        ;; Did we loop back to 0x00? if so, halt.
        EQU.num 0x00
        JUM.num :halt
        
        ;; Show the value in that ram position
        LDD.ptr counter
        STD.out
        
        ;; Go back to the start
        LDD.num 0xff
        JUM.num start
        
        ;; Halt.
halt    JUM.num :halt
```

### Includes

Files can be included into other files, like in C:

```asm
#inc    firstinclude.asm
#inc    secondinclude.asm
```

Beware of include loops! The assembler doen't check for them yet!

## Instructions and modifiers

| Instruction | Meaning               |                                                                                                                                                |
|-------------|-----------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| NOI         | **NO I**nstruction    | Do nothing.                                                                                                                                    |
| JUM         | **JUM**p              | Jump to some place.                                                                                                                            |
| LDD         | **L**oa**D D**ata     | Load from source into the DTA registry.                                                                                                        |
| STD         | **ST**ore **D**ata    | Store whatever is in the DTA registry in the destination.                                                                                      |
| LDA         | **L**oa**D A**ddress  | Load from source into the ADR registry.                                                                                                        |
| STA         | **ST**ore **A**ddress | Store whatever is in the ADR registry in the destination.                                                                                      |
| ADD         | **ADD**               | Add together the contents of the DTA and AUX registries, store the result in DTA.                                                              |
| SUB         | **SUB**stract         | Substract to the contents of the DTA resgistry the contents of the AUX registry, store the result in DTA.                                      |
| NAN         | **NAN**d              | Perform the NAND bitwise operation with contents of the DTA and AUX registries, store the result in DTA.                                       |
| SHL         | **SH**ift **L**eft    | Shift left the contents of the DTA registry. Fill with 0s.                                                                                     |
| SHR         | **SH**ift **R**ight   | Shift right the contents of the DTA registry. Fill with 0s.                                                                                    |
| EQU         | check **EQU**al       | Check if the contents of the DTA and AUX registries are equal, store 0xff in DTA if they are and 0x00 otherwise.                               |
| GRE         | check **GRE**ater     | Check if the contents of the DTA registry are greater than the contents of the AUX registry, store 0xff in DTA if they are and 0x00 otherwise. |
