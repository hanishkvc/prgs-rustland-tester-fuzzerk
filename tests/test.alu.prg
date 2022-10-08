# Test alu operations
#

	jump START

!label START
	letint i01 $10
	letint i02 $2

	iobnew term console
	letbuf msg1 "Staring on the journey"
	iobwrite term msg1

	add radd i01 i02
	sub rsub i01 i02
	mult rmult i01 i02
	div rdiv i01 i02
	mod rmod i01 i02

	letbuf msg1 "End of the journey"
	iobwrite term msg1

