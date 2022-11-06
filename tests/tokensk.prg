#
# Test out tokensk bsed flexibilty
#
	jump	START


!func PRINT_ME smsg
	iobwrite	term	smsg
	ret


!func TEST_ADD
	letglobal	int1	48
	add		res1	int1	1
	add		res2	int1	"2"
	add		res3	int1	!be(0x0403020100, 3)
	letlocal	index	4
	add		res4	int1	!be(  0x0403020100  ,   index  )
	bufmerged	smsg	"Add+:res1 = int1 + 20:" res1 "\n"
	call		PRINT_ME smsg
	#bufmerged	smsg	"Add+:res2 = int1 + \"2\":" res2 "\n"
	#bufmerged	smsg	"Add+:res2 = int1 + \\\"2\\\":" res2 "\n"
	bufmerged	smsg	"Add+:res2 = int1 + str(2):" res2 "\n"
	call		PRINT_ME smsg
	bufmerged	smsg	"Add+:res3 = int1 + !be(0x0403020100, 3):" res3 "\n"
	call		PRINT_ME smsg
	bufmerged	smsg	"Add+:res4 = int1 + !be(  0x0403020100  , index=4  ):" res4 "\n"
	call		PRINT_ME smsg
	ret


!func TEST_ERRR
	letlocal	lint	0x30313233
	letlocal	lint	!be(   0x30313233  ,   2    )
	letlocal	lint	! be (   0x30313233     ,    2   )
	ret


!label START
	iobnew		term	console
	call		TEST_ADD
	call		TEST_ERRR
	end

