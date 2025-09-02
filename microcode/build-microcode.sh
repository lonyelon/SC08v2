#!/bin/sh

grep "^[^;][^;]" microcode | sed '
	s/\./\t/

	s/NOI/0/
	s/JUM/1/
	s/LDD/2/
	s/STD/3/
	s/LDA/4/
	s/STA/5/
	s/ADD/8/
	s/SUB/9/
	s/NAN/10/
	s/SHL/11/
	s/SHR/12/
	s/EQU/13/
	s/GRE/14/

	s/noa/0/
	s/num/1/
	s/ram/2/
	s/rom/3/
	s/ptr/4/
	s/prr/5/
	s/inp/6/
	s/out/7/

	s/DTA S/1/
	s/ADR S/2/
	s/AUX S/4/
	s/INP E/8/
	s/OUT S/16/
	s/ALU E/32/
	s/RAM S/64/
	s/RAM E/128/
	s/ROM S/256/
	s/ROM E/512/
	s/PRC I/1024/
	s/PRC S/2048/
	s/INS S/4096/
	s/SWT/8192/

	s/,/+/g
	' | awk '
	BEGIN{
		FS="\t";
		OFS=FS
	}
	{
		print $1*(2^7)+$2*(2^3)+$3, $4}
	'
