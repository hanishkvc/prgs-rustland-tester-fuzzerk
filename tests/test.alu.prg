# Test alu operations
#

	jump START

!func PRINT_ALUVARS
	bufmerged.s bmsg "Results :RAdd:" radd ":RSub:" rsub ":RMult:" rmult ":RDiv:" rdiv ":RMod:" rmod "\n"
	iobwrite term bmsg
	ret

!label START
	letint i01 10
	letint i02 2

	iobnew term console
	iobwrite term "Starting on the journey\n"

	add radd i01 i02
	sub rsub i01 i02
	mult rmult i01 i02
	div rdiv i01 i02
	mod rmod i01 i02
	call PRINT_ALUVARS

	sub rsub i01 12
	mult rmult i01 rsub
	bufmerged.s msgIn "Sub using " i01 " and " 12 "\nMult using " i01 " and rsub\n"
	iobwrite term msgIn
	call PRINT_ALUVARS

	sub rsub i01 0x12
	mult rmult i01 rsub
	mod rmod 0x15 i02
	bufmerged.s msgIn "Sub using " i01 " and " 0x12 "\nMult using " i01 " and rsub\nMod using " 0x15 " and " i02 "\n"
	iobwrite term msgIn
	call PRINT_ALUVARS

	iobwrite term "End of the journey\n"

