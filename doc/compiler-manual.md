# How does the compiler work

The compiler adds another layer of abstraction on top of the assembler, allowing for the use of a LISP-like programming language instead of the assembly language.

The compiler evaluates every expression bottom-up, meaning that, in code like:

```lsp
(if (> a b) (- a (+ 3 2)))
```

The first element to get evaluated is `(+ 3 2)`.

## Instruction translation

### If X do Y

```lisp
(if X Y)
```
```asm
        X
        EQU.num 0x00
        JUM.num :endif
        Y
:endif  ...
```

### If X do Y, if not, do Z

```lisp
(if X Y Z)
```
```asm
        X
        EQU.num 0x00
        JUM.num :else
        Y
        LDD.num 0xFF
        JUM.num :endif
:else
        Z
:endif  ...
```

### While X do Y

```lsp
(while X Y)
```
```asm
:check    X
          EQU.num 0x00
          JUM.num :endwhile
          Y
          LDD.num 0xFF
          JUM.num :check
:endwhile ...
```

### Instruction list

```lsp
(ins A B C D ...)
```
```asm
        A
        B
        C
        D
        ...
```

### Variable name assignment

```
(var 1 a X)
```
```
#var    a 1
        X
#endvar
```
