;; Add three numbers together.
	LDD.num	0x01
	STD.ram	0x00
	LDD.rom	:FIRST
	STD.ram	0x01
	LDD.rom	:SECOND
	ADD.ram	0x00
	ADD.ram	0x01
	STD.out

;; Number values
FIRST	0x05
SECOND	0x0A
