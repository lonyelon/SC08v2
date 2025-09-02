#var	counter	0x00
#var	buff	0x01

INP	LDD.inp
	STD.ram buff
	GRE.num 0x3e
	JUM.num :CONVERT
	LDD.ram buff
	STD.ptr counter
	
	LDD.ram counter
	ADD.num 0x01
	STD.ram counter
	
	LDD.num 0xff
	JUM.num :INP

CONVERT	LDD.ram counter
	SUB.num 0x01
	LDD.ptr counter

	JUM.num :CONVERT
