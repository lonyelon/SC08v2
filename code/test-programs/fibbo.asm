;; Fibbonacci secuence calculator (up o 233 for byte limitations). This program
;; returns the nth Fibbonacci number.

#var	arg	0x00
#var	first	0x01
#var	second	0x02

;; Ask the user for n.
	LDD.inp
	STD.ram	arg
	GRE.num	2
	JUM.num	:start
	LDD.num	1
	JUM.num	:end

;; Fill memory with correct values to start the calculation. Doing this with "2"
;; and "3" instead of two ones and...
start	LDD.num	2
	STD.ram	first
	LDD.num	3
	STD.ram	second

;; ...doing this first, the index update, allows to save at least two operations.
loop	LDD.ram	arg
	SUB.num	1
	STD.ram	arg
	EQU.num	2
	JUM.num	:print1st
	LDD.ram	arg
	SUB.num	1
	STD.ram	arg
	EQU.num	2
	JUM.num	:print2nd

;; Main calculation.
	LDD.ram	first
	ADD.ram	second
	STD.ram	first
	ADD.ram	second
	STD.ram	second
	LDD.num	0xff
	JUM.num	:loop


;; Print numbers and end.
print1st	LDD.ram	first
	JUM.num	:end
print2nd	LDD.ram	second
	JUM.num	:end
end	STD.out
halt	JUM.num	:halt
