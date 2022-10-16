#var	first	0x00
#var	second	0x01
#var	buffer	0x02
#var	counter	0x03
#var	result	0x04

;;  Get both numbers from the user.
	LDD.inp
	STD.ram	first
	LDD.inp
	STD.ram	second

;; If the SECOND number is smaller than the FIRST, change the position of both of
;; them to save loop iterations.
	GRE.ram	first
	JUM.num	:START
	LDD.ram	first
	STD.ram	buffer
	LDD.ram	second
	STD.ram	first
	LDD.ram	buffer
	STD.ram	second

;; Multiply both numbers by counting up to FIRST, which should be the smaller
;; number after the previous step.
START	LDD.ram	counter
	ADD.num	0x01
	STD.ram	counter
	GRE.ram	first
	JUM.num	:END
	LDD.ram	result
	ADD.ram	second
	STD.ram	result
	JUM.num	:START

;; Print result.
END	LDD.ram	result
	STD.out

;; Halt.
HALT	JUM.num	:HALT
