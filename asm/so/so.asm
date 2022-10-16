;; Global variables
#var	RVAL	0x01
#var	LVAL	0x02
#var	CALL_COUNTER	0x03

;; SO start, go to shell
:SYS_START	JUM.num	:SHELL

;; Include external programs
#inc	free_memory.asm

:RETURN	STD.ram RVAL
	LDD.ram CALL_COUNTER
	SUB.num 0x02
	STD.ram CALL_COUNTER
	LDA.ptr CALL_COUNTER
	LDD.num 0xff
	JUM.ptr CALL_COUNTER

;; Shell
#spc	SHELL
#def	inp	0x00
:SHELL	LDD.inp
	STD.ram inp
#end SHELL

;; Shell data
;; Format N TO STR[0] .., STR[N]
;;:shell_data_start	0x00 :PROJ_MULT
;;	0x00 0x00
;;	0x00 0x00

#spc	MULT
#def	counter	0x80
#def	num	0x81
#def	result	0x82
:PROG_MULT	LDD.inp
	STD.ram counter
	LDD.inp
	STD.ram num
	LDD.num counter
	STD.ram result
:mult_start	LDD.ram counter
	EQU.num 0x00
	JUM.num :mult_end
	SUB.num 0x01
	LDD.ram result
	ADD.ram num
	STD.ram result
	LDD.num 0xFF
	JUM.num :mult_start
:mult_end	LDD.ram result
#end MULT	STD.out

;; Safe sum: add two characters and take into account overflows.
#spc	SAFE_SUM
	LDD.ram LVAL0
	ADD.ram LVAL1
	_RET
#end	SAFE_SUM
