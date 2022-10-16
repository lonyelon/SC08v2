;; Init.
        JUM.num :LOAD

;; Variables
NUMBER  0x30

;; Load variables.
LOAD    LDD.rom :NUMBER
        STD.ram 0x02
        LDD.num 0x01
        STD.ram 0x01
        JUM.num :START

;; Main Loop
START   LDD.ram 0x00
        ADD.ram 0x01
        STD.ram 0x00
        EQU.ram 0x02
        JUM.num :END
        LDD.num 0xff
        JUM.num :START

;; Print vars and HALT.
END     LDD.ram 0x00
        STD.out
        LDD.ram 0x01
        STD.out
        LDD.ram 0x02
        STD.out
HALT    JUM.num :HALT
