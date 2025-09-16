# How does the compiler work

The compiler adds another layer of abstraction on top of the assembler, allowing for the use of a LISP-like programming language (`sclsp`) instead of the assembly language.

The compiler evaluates every expression bottom-up, meaning that, in code like:

```lsp
(if (> a b) (- a (+ 3 2)))
```

The first element to get evaluated is `(+ 3 2)`.

Different expressions have different rules.
For example, an `if` expression like `(if X Y)` gets translated to:
```asm
        X               ;; Run X first
        EQU.num 0x00    ;; Invert the value
        JUM.num :endif  ;; Jump if X condition was not met (thats why the value was inverted)
        Y               ;; Execute Y if X was met
:endif  NOI.noa         ;; Placeholder instruction to ensure a destination point for the jump
```
