# ssc08v02 instructions and modifiers

The table below explains what every instruction in the CPU does.
Every byte value not in the list is assumed to be `NOI.noa`, no instruction.

| Name | Byte       | Value                                               |
| ---- | ---------- | --------------------------------------------------- |
| STA  | 00 000 mmm | Store data from the `DTA` registry.                 |
| LDA  | 01 000 mmm | Load data into the `DTA` registry.                  |
| JUM  | 10 000 mmm | Conditional jumps. Only jumps if `DTA` holds `0xFF` |
| NAN  | 11 001 mmm | Bitwise NAND operation between `DTA` and `DTB`.     |
| ADD  | 11 010 mmm | Add `DTA` and `DTB`.                                |
| SUB  | 11 011 mmm | Substract `DTB` from `DTA`.                         |
| EQU  | 11 100 mmm | Check if `DTA` and `DTB` are equal.                 |
| GRE  | 11 101 mmm | Check if `DTA` > `DTB`.                             |
| SHL  | 11 110 mmm | DTA << 1                                            |
| SHR  | 11 111 mmm | DTA >> 1                                            |

As we can see, every instruction has two bits for the ID, three for the ALU operation (has to be `000` in non-ALU instructions to allow reading from DTA), and three for the modifier (source/destination of data).
The following is a list of modifiers:

| Name [extra bytes] | Code | Description |
| ------------------ | ---- | ----------- |
| `noa`              | 000  | Do nothing (*No* *a*ction). If used with `LDA` then load the `INP_FLAG` registry. |
| `num #1 [#2]`      | 001  | Hardcoded byte (could be 2 bytes for ROM operations like JUM).                    |
| `ram #1`           | 010  | RAM at ´#1´.     |
| `rom #1 #2`        | 011  | ROM at ´#1 | #2´. |
| `prr`              | 100  | RAM at RAM ´DTA´ (pointer) |
| `ptr #1`           | 101  | RAM at RAM ´#1´ (pointer) |
| `inp`              | 110  | INP register. |
| `out`              | 111  | OUT register. |

# Valid instruction-modifier combinations

| Instruction     | Action |
| --------------- | ------ |
| `STD.noa`       | NOI.noa: 0000.0000 tiene que no hacer nada |
| `STD.num`       | NOI.noa: no se puede guardar datos en un número |
| `STD.ram #1`    | RAM[#1] = DTA |
| `STD.rom #1 #2` | ROM[#1][#2] = DTA |
| `STD.prr`       | RAM[DTA] = DTA |
| `STD.ptr #1`    | RAM[#1] = DTA |
| `STD.inp`       | NOI.noa: no se puede guardar datos en entrada |
| `STD.out`       | OUT = DTA |
|                 | |
| `LDD.noa`       | DTA = INP_FLAG |
| `LDD.num #1`    | DTA = #1 |
| `LDD.ram #1`    | DTA = RAM[#1] |
| `LDD.rom #1 #2` | DTA = ROM[#1][#2] |
| `LDD.prr`       | DTA = RAM[DTA] |
| `LDD.ptr #1`    | DTA = RAM[#1] |
| `LDD.inp`       | DTA = INP |
| `LDD.out`       | DTA = OUT_FLAG |
|                 | |
| `JUM.noa`       | NOI.noa: no tiene sentido |
| `JUM.num #1 #2` | PC = #1 #2 |
| `JUM.ram #1 #2` | PC = RAM[#1] RAM[#2] |
| `JUM.rom`       | NOI.noa? Solo valida si la ROM tine 2 bytes de dirección, PC S fuerza SWT y max(PC) = 15 |
| `JUM.prr #1`    | NOI.noa? PC = RAM[DTA] | RAM[#1] |
| `JUM.ptr #1 #2` | NOI.noa? PC = RAM[#1] | RAM[#2] |
| `JUM.inp #1`    | NOI.noa? PC = INP | #1 |
| `JUM.out`       | NOI.noa: no se puede leer del registro de salida |
|                 | |
| `ALU.noa`       | NOI.noa? DTA = DTA op DTB |
| `ALU.num #1`    | DTA = DTA op #1 |
| `ALU.ram #1`    | DTA = DTA op RAM[#1] |
| `ALU.rom #1 #2` | DTA = DTA op ROM[#1][#2] |
| `ALU.prr`       | DTA = DTA op RAM[DTA] |
| `ALU.ptr #1`    | DTA = DTA op RAM[#1] |
| `ALU.inp`       | DTA = DTA op INP |
| `ALU.out`       | NOI.noa: no se puede leer dal registro de salida |

# Combinaciones frecuentes

El valor de un puntero *ejemplo se obtiene de forma trivial con:
```asm
	LDD.ptr	ejemplo
```
Pero si queremos usar punteros de punteros (***ejemplo), podemos:
```asm
	LDD.ptr ejemplo
	LDD.prr
	LDD.prr
```
