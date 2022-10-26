#
# A intro program
#

	#
	# Jump to create some space to put required code in between
	#
	jump StArT


#
# Check out variables
#
!func VARIABLES
	call		PRINT_ME "\n\n **** Variables **** \n\n"
	letint 		MeInt 101
	letstr 		MeStr "Me a String"
	letbuf 		MeBuf "I can have any binary value"
	letbuf 		MeBuf $0x3031322D6362612D373839
	bufmerged.s 	AMsg "The Vars are:StringMode:\n" "\tInt:" MeInt "\n\tStr:" MeStr "\n\tBuf:" MeBuf "\n"
	iobwrite 	term AMsg
	bufmerged.b	AMsg "The Vars are:BinBufMode:\n" "\tInt:" MeInt "\n\tStr:" MeStr "\n\tBuf:" MeBuf "\n"
	iobwrite 	term AMsg
	bufmerged.b	AMsg "The Vars are:StrXCasts :\n" "\tInt:" !str(MeInt) "\n\tStr:" MeStr "\n\tBuf:" !str(MeBuf) "\n"
	iobwrite 	term AMsg
	bufmerged.b	AMsg "The Vars are:HexXCasts :\n" "\tInt:" !strhex(MeInt) "\n\tStr:" !strhex(MeStr) "\n\tBuf:" !strhex(MeBuf) "\n"
	iobwrite 	term AMsg
	ret


!func GLOBAL_VARS
	call		PRINT_ME "\n\n **** Global Variables **** \n\n"

	# Global vars using explicit lettype var setting
	letint		gint1 201
	letstr		gstr1 "Global Vars Set1"
	letbuf		gbuf1 $0x393138323733363435
	bufmerged.s	tmsg1 "GlobalsLetType:Str     : Int[" gint1 "] Str[" gstr1 "] Buf[" gbuf1 "]\n"
	call PRINT_ME tmsg1
	bufmerged.b	tmsg1 "GlobalsLetType:Buf     : Int[" gint1 "] Str[" gstr1 "] Buf[" gbuf1 "]\n"
	call PRINT_ME tmsg1
	bufmerged	tmsg1 "GlobalsLetType:StrXCast: Int[" !str(gint1) "] Str[" gstr1 "] Buf[" !str(gbuf1) "]\n"
	call PRINT_ME tmsg1

	# Global vars using explicit letglobal.type var setting
	letglobal.i	gint2 202
	letglobal.s	gstr2 "Global Vars Set2"
	letglobal.b	gbuf2 $0x393831323733363435
	bufmerged.s	tmsg2 "GlobalsLetGlobal.Type:Str: Int[" gint2 "] Str[" gstr2 "] Buf[" gbuf2 "]\n"
	call PRINT_ME tmsg2
	bufmerged.b	tmsg2 "GlobalsLetGlobal.Type:Buf: Int[" gint2 "] Str[" gstr2 "] Buf[" gbuf2 "]\n"
	call PRINT_ME tmsg2

	# Global vars using implicit letglobal var setting
	letglobal	gint3 203
	letglobal	gstr3 "Global Vars Set3"
	letglobal	gbuf3 $0x393837363132333435
	bufmerged.s	tmsg3 "GlobalsLetGlobal:Str: Int[" gint3 "] Str[" gstr3 "] Buf[" gbuf3 "]\n"
	call PRINT_ME tmsg3
	bufmerged.b	tmsg3 "GlobalsLetGlobal:Buf: Int[" gint3 "] Str[" gstr3 "] Buf[" gbuf3 "]\n"
	call PRINT_ME tmsg3
	ret


#
# Check out few if condition instructions
#

!func CC_GOOD
	add CheckInt CheckInt 1
	ret

!func CC_BAD
	add CheckInt CheckInt 1
	ret

!func CC_CHECK
	ifge CheckInt 2 goto CCC_BAD
	letbuf tMsg "Yes IfLT seems fine\n"
	iobwrite term tMsg
	jump CCC_RET
!label CCC_BAD
	letbuf tMsg "No IfLT seems messed up\n"
	iobwrite term tMsg
!label CCC_RET
	ret

!func CONDITIONS
	call PRINT_ME "\n\n **** Conditions **** \n\n"
	letint CheckInt 0
	iflt 0 1 call CC_GOOD
	ifgt 0 1 call CC_BAD
	call CC_CHECK
	ret


#
# Check function arg mechanism
#

!func PRINT_ME msg
	iobwrite term msg
	ret

!func CHECK_FARG_L1 arg1
	call PRINT_ME arg1
	ret

!func CHECK_FARG_MULTI str1 int1
	add		intres int1 20
	bufmerged.s	tmsg "\n\nFArg:Multi:Arg1:" str1 "\n" "FArg:Multi:Arg2:Adjusted int1 is:" intres "\n\n"
	call		PRINT_ME tmsg
	ret

!func CHECK_FARG_REF_LOCAL_OK
	letlocal	lsrc1		"FArg:LocalDirect: FArg passed can be a local var\n"
	call		PRINT_ME	lsrc1
	letstr		lsrc2		"FArg:LocalL1: FArg passed can be local, which inturn can be passed to another func\n"
	call		CHECK_FARG_L1	lsrc2
	ret

!func CHECK_FARG_LITERALS_SIMPLE1
	call		PRINT_ME "FARGS: Test literals passing 1\n"
	ret

!func CHECK_FARG_LITERALS_SIMPLE2 msg
	call		PRINT_ME msg
	ret

!func CHECK_farg_literals_multi arg1 arg2 yesArg3 arg4
	bufmerged.s tmsg "FARGS:LiteralsMulti:\n\t" arg1 "\n\t" arg2 "\n\t" yesArg3 "\n\t" arg4 "\n"
	call PRINT_ME tmsg
	ret


!func CHECK_FARGS
	call PRINT_ME "\n\n **** Function args **** \n\n"
	letstr	fargtest1 "FArgs: This is a global var passed to func using func args\n"
	call	CHECK_FARG_L1 fargtest1
	letstr	fargtest2 "FArgs: This is another global var passed to func using func args\n"
	call	CHECK_FARG_L1 fargtest2
	letint	testint1 100
	call	CHECK_FARG_MULTI fargtest1 testint1
	call	CHECK_FARG_REF_LOCAL_OK
	call	CHECK_FARG_LITERALS_SIMPLE1
	call	CHECK_FARG_LITERALS_SIMPLE2 "FARGS: Test literals passing 2\n"
	call	CHECK_farg_literals_multi 10244201 "Test literals passing 3" testint1 "thank you"
	ret


#
# Check function local vars
#

!func LOCAL_VARS_FARG_NOINFER testme
	letlocal xvar testme
	ret

!func LOCAL_VARS_OK
	# Explicitly specified local var types
	letlocal.i lint 987
	letlocal.s lstr "hello world, me local"
	letlocal.b lbuf $0x30313233353436373839
	bufmerged.s tmsg "LocalsOk:DestStr: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	bufmerged.b tmsg "LocalsOk:DestBuf: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	# Implicitly infered local var types, based on assigned literal value; Compile time
	letlocal oint 789
	letlocal ostr "me also string"
	letlocal obuf $0x303132
	bufmerged.s tmsg "LocalsOkAutoCompile:DestStr: Int[" oint "] Str[" ostr "] Buf[" obuf "]\n"
	call PRINT_ME tmsg
	bufmerged.b tmsg "LocalsOkAutoCompile:DestBuf: Int[" oint "] Str[" ostr "] Buf[" obuf "]\n"
	call PRINT_ME tmsg
	# Implicitly infered local var types, based on assigned var; Run time
	letlocal xint oint
	letlocal xstr ostr
	letlocal xbuf obuf
	bufmerged.s tmsg "LocalsOkAutoRuntime:DestStr: Int[" xint "] Str[" xstr "] Buf[" xbuf "]\n"
	call PRINT_ME tmsg
	bufmerged.b tmsg "LocalsOkAutoRuntime:DestBuf: Int[" xint "] Str[" xstr "] Buf[" xbuf "]\n"
	call PRINT_ME tmsg
	ret

!func LOCAL_VARS_INT
	letlocal lint 0x55555555
	letlocal oint 123
	add gint lint oint
	bufmerged.s printme "GInt:" gint " lint:" lint " oint:" oint "\n"
	call PRINT_ME printme
	ret

!func LOCAL_VARS_IN
	letlocal lint 123
	letlocal lstr "hello me local in"
	letlocal lbuf $0x39383736353433323130
	bufmerged.s tmsg "LocalsIN:DestStr: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	bufmerged.b tmsg "LocalsIN:DestBuf: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	ret

!func LOCAL_VARS
	call PRINT_ME "\n\n **** Local Vars **** \n\n"
	letlocal lint 565
	letlocal lstr "hello me local"
	letlocal lbuf $0x30313233343536373839
	call LOCAL_VARS_IN
	bufmerged.s tmsg "Locals:DestStr:  Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	bufmerged.b tmsg "Locals:DestBuf:  Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	letint gint 565
	bufmerged.b tmsg "Locals:DestBuf: GInt[" gint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	call LOCAL_VARS_OK
	call LOCAL_VARS_INT
	ret


#
# Check xcasting and indexing
#

!func XCASTS
	letlocal.s lstr "\n\n\t 123 \n\t\n\t"
	letlocal.i lint lstr
	bufmerged  bmsg "XCASTS:0: lstr [" lstr "], str(lstr)[" !str(lstr) "], hex(lstr)[" !strhex(lstr) "] hex(trim(lstr))[" !strhex(!strtrim(lstr)) "]\n"
	call       PRINT_ME bmsg
	bufmerged  bmsg "XCASTS:1: lstr [" lstr "], lint[" !str(lint) "], lintHex[" !strhex(lint) "]\n"
	call       PRINT_ME bmsg
	ret

!func INDEX
	letlocal.s	lstr "AbCdEf"
	bufmerged	bmsg "INDEX:FromString:raw: 0[" lstr[0] "] 1[" lstr[1] "]\n"
	call		PRINT_ME bmsg
	bufmerged	bmsg "INDEX:FromString:str: 0[" !str(lstr[0]) "] 1[" !str(lstr[1]) "]\n"
	call		PRINT_ME bmsg
	letlocal.i	lint 0x303132
	bufmerged	bmsg "INDEX:FromInt:raw: 0[" lint[0] "] 1[" lint[1] "]\n"
	call		PRINT_ME bmsg
	bufmerged	bmsg "INDEX:FromInt:str: 0[" !str(lint[0]) "] 1[" !str(lint[1]) "]\n"
	call		PRINT_ME bmsg
	ret


#
# Alu ops
#

!func SHIFTS
	letlocal.i	lint1 0x505050
	letlocal.i	lint2 0x020202
	slb		lsl lint1 lint2
	srb		lsr lint1 lint2
	bufmerged	bmsg "SHIFTS: lint1[" !strhex(lint1) "] lint2[" !strhex(lint2) "] slb[" !strhex(lsl) "] srb[" !strhex(lsr) "]\n"
	call		PRINT_ME bmsg
	letlocal.i	lint1 0x050505
	letlocal.i	lint2 0x020202
	slb		lsl lint1 lint2
	srb		lsr lint1 lint2
	bufmerged	bmsg "SHIFTS: lint1[" !strhex(lint1) "] lint2[" !strhex(lint2) "] slb[" !strhex(lsl) "] srb[" !strhex(lsr) "]\n"
	call		PRINT_ME bmsg
	ret


#
# Initialise
#
!func INIT
	iobnew term console
	ret


#
# The practical Program entry point
#
!label StArT
	call INIT
	#letglobal tbuf $0x1234567
	#letlocal lstr "will fail"
	call VARIABLES
	call GLOBAL_VARS
	call CONDITIONS
	call CHECK_FARGS
	call LOCAL_VARS
	call XCASTS
	call INDEX
	call SHIFTS
	call PRINT_ME "\n**** Reached end of the program ****\n"
	end
