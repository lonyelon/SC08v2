#var	first	0x00
#var	second	0x01
#var	buffer	0x02
#var	counter	0x03
#var	result0	0x04
#var	result1	0x05

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
	LDD.ram	result0
	ADD.ram	second
	STD.ram	buffer
	GRE.ram	result0
	JUM.num	:STORE
	LDD.ram	result1
	ADD.num	0x01
	STD.ram	result1
STORE	LDD.ram	buffer
	STD.ram	result0
	LDD.num	0xff
	JUM.num	:START

;; Print result.
END	LDD.ram	result0
	STD.out
	LDD.ram	result1
	STD.out

;; Halt.
HALT	JUM.num	:HALT
