# Test alu operations
#

	jump START

!label START
	letint i01 $10
	letint i02 $2

	iobnew term console
	letbuf msgnl 0x0A
	letbuf msg1 "Staring on the journey"
	bufsmerge msgStart msg1 msgnl
	letbuf msg1 "End of the journey"
	bufsmerge msgEnd msg1 msgnl
	iobwrite term msgStart

	add radd i01 i02
	sub rsub i01 i02
	mult rmult i01 i02
	div rdiv i01 i02
	mod rmod i01 i02

	letbuf.s bradd radd
	letbuf.s brsub rsub
	letbuf msgpre "RAdd:"
	letbuf msgbtw ":RSub:"
	bufsmerge bmsg msgpre bradd msgbtw brsub msgnl
	iobwrite term bmsg

	iobwrite term msgEnd

