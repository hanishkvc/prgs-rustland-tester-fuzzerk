
	jump START_HERE

!func test_int_ok
	letlocal.i	ii	123
	letlocal.i	is	"456"
	letlocal.i	ib	$0x5a5a5a5aa5a5a5a5
	bufmerged	bmsg	"TestIntOK: ii[" ii "] is[" is "] ib[" ib "]\n"
	iobwrite	term	bmsg
	bufmerged	bmsg	"TestIntOK: ii[" !str(ii) "] is[" !str(is) "] ib[" !str(ib) "]\n"
	iobwrite	term	bmsg
	bufmerged	bmsg	"TestIntOK: ii[" !strhex(ii) "] is[" !strhex(is) "] ib[" !strhex(ib) "]\n"
	iobwrite	term	bmsg
	ret

!func test_int_notok
	letlocal.i	is	"abc"
	letlocal.i	ibless	$0x5a5a5a5aa5a5a5
	letlocal.i	ibmore	$0x5a5a5a5aa5a5a5a512
	ret

!func test_specials
	letlocal.b	brb	__RANDOM__BYTES__4
	#letlocal.i	irb	__RANDOM__BYTES__10
	letlocal.i	irb	__RANDOM__BYTES__4
	bufmerged	bmsg	"TestSpecials: brb4[" !str(brb) "] irb4[" !str(irb) "]\n"
	iobwrite	term	bmsg
	bufmerged	bmsg	"TestSpecials: brb4[" !strhex(brb) "] irb4[" !strhex(irb) "]\n"
	iobwrite	term	bmsg
	ret


!label START_HERE
	iobnew	term console
	call	test_int_ok
	#call	test_int_notok
	call	test_specials

