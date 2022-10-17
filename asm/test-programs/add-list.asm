;; Get and store a list of integers given by the user and then add them in order.
;; To end the list just input a 0.
;; TODO Check when we get to the end of the ram (pointer = 0xff).

#var	pointer	0x00
#var	result	0x01

;; Se the initial value for the list pointer, in this case 2 since we have two
;; values already in RAM, the pointer and the result.
	LDD.num	2
	STD.ram	pointer

;; Ask the user for the numbers and store them in RAM. Only stop once the user
;; gives a zero.
ASK_LOOP	LDD.inp
	STD.prr	pointer
	EQU.num	0
	JUM.num	:SHOW
	LDD.ram	pointer
	ADD.num	1
	STD.ram	pointer
	LDD.num	0xff
	JUM.num	:ASK_LOOP

;; Go back in the list adding the numbers up.
SHOW	LDD.prr	pointer
	ADD.ram	result
	STD.ram	result
	LDD.ram	pointer
	SUB.num	1
	STD.ram	pointer
	EQU.num	result
	JUM.num	:OUT
	LDD.num	0xff
	JUM.num	:SHOW

;; Show result of the operation.
OUT	LDD.ram	result
	STD.out

;; Halt.
HALT	JUM.num	:HALT
