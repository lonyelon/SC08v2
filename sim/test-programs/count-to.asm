#var    buff    0x00
#var    dest    0x01

;; Init.
        JUM.num :LOAD

;; Variables
NUMBER  0x30

;; Load variables.
LOAD    LDD.rom :NUMBER
        STD.ram dest
        JUM.num :START

;; Main Loop
START   LDD.ram buff
        ADD.num 0x01
        STD.ram buff
        EQU.ram dest
        JUM.num :END
        LDD.num 0xff
        JUM.num :START

;; Print vars and HALT.
END     LDD.ram buff
        STD.out
        LDD.ram dest
        STD.out

;; HALT
HALT    JUM.num :HALT
