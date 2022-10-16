;; This call copies all data in user space and moves it to ROM, so other
;; processes can take over and use that RAM without any collisions.

#var	COUNTER 0x00
#var	SPACE0 0x01
#var	SPACE1 0x02

	;; Load config
:FREE_MEMORY	LDD.num 0x80
	STD.ram COUNTER
	LDD.num 0x0F
	STD.ram SPACE0
	LDD.num 0x00
	STD.ram SPACE1

	;; Save to ROM and clean RAM
:start	LDD.ptr COUNTER
	LDA.ram SPACE0
	STD.rom SPACE1
	LDD.num 0x00
	STD.ptr COUNTER

	;; Update counter
	LDD.ram COUNTER
	ADD.num 0x01
	STD.ram COUNTER

	;; Update SPACE
	LDD.ram SPACE1
	EQU.num 0xff
	JUM.num :stop
	LDD.ram SPACE1 ;; Si no es 255, sumamos 1.
	ADD.num 0x01
	STD.ram SPACE1
	LDD.num 0xff
	JUM.num :final_check
:stop	LDA.ram SPACE0
	ADD.num 0x01
	STD.ram SPACE0
	LDD.num 0x00
	STD.RAM SPACE1

	;; Are we finished?
:final_check	LDD.ram COUNTER
	GRE.num 0x00
	JUM.num :start
