# Test alu operations
#

	jump START

!label PRINT_ALUVARS
	letbuf.s bradd radd
	letbuf.s brsub rsub
	letbuf.s brmult rmult
	letbuf.s brdiv rdiv
	letbuf.s brmod rmod
	letbuf msgpre "RAdd:"
	letbuf msgbtw1 ":RSub:"
	letbuf msgbtw2 ":RMult:"
	letbuf msgbtw3 ":RDiv:"
	letbuf msgbtw4 ":RMod:"
	bufsmerge bmsg msgpre bradd msgbtw1 brsub msgbtw2 brmult msgbtw3 brdiv msgbtw4 brmod msgnl
	iobwrite term bmsg
	ret

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
	call PRINT_ALUVARS

	sub rsub i01 $12
	mult rmult i01 rsub
	call PRINT_ALUVARS

	sub rsub i01 $0x12
	mult rmult i01 rsub
	mod rmod $0x15 i02
	call PRINT_ALUVARS

	iobwrite term msgEnd

