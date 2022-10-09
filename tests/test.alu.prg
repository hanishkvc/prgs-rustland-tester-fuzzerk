# Test alu operations
#

	jump START

!label PRINT_ALUVARS
	letbuf.s bradd radd
	letbuf.s brsub rsub
	letbuf.s brmult rmult
	letbuf.s brdiv rdiv
	letbuf.s brmod rmod
	letbuf msgArg1 "Arg1:"
	letbuf msgArg2 ":Arg2:"
	letbuf msgAdd ":RAdd:"
	letbuf msgSub ":RSub:"
	letbuf msgMult ":RMult:"
	letbuf msgDiv ":RDiv:"
	letbuf msgMod ":RMod:"
	bufsmerge bmsg msgAdd bradd msgSub brsub msgMult brmult msgDiv brdiv msgMod brmod msgnl
	iobwrite term bmsg
	ret

!label START
	letint i01 10
	letint i02 2

	iobnew term console
	# letbuf msgnl 0x0A, will work, but will have extra unwanted 00 bytes wrt native isize bytes size
	letbuf msgnl $0x0A
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

	sub rsub i01 12
	mult rmult i01 rsub
	call PRINT_ALUVARS

	sub rsub i01 0x12
	mult rmult i01 rsub
	mod rmod 0x15 i02
	call PRINT_ALUVARS

	iobwrite term msgEnd

