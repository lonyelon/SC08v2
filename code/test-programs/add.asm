#var	FIRST	0x00
#var	SECOND	0x01

;; Add three numbers together.
	LDD.rom	:FIRST
	STD.ram	FIRST
	LDD.rom	:SECOND
	STD.ram	SECOND
	LDD.rom	:THIRD
	ADD.ram	FIRST
	ADD.ram	SECOND
	STD.out
HALT	JUM.num	:HALT

;; Number values
FIRST	0x05
SECOND	0x0A
THIRD	0x10
