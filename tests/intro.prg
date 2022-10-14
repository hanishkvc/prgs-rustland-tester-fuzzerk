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
	letint MeInt 101
	letstr MeStr "Me a String"
	letbuf MeBuf "I can have any binary value"
	letbuf MeBuf $0x3031322D6362612D373839
	bufmerged.s AMsg "The Vars are:StringMode:\n" "\tInt:" MeInt "\n\tStr:" MeStr "\n\tBuf:" MeBuf "\n"
	iobwrite term AMsg
	bufmerged.b AMsg "The Vars are:BinBufMode:\n" "\tInt:" MeInt "\n\tStr:" MeStr "\n\tBuf:" MeBuf "\n"
	iobwrite term AMsg
	ret


!func GLOBAL_VARS
	# Global vars using explicit lettype var setting
	letint gint1 201
	letstr gstr1 "Global Vars Set1"
	letbuf gbuf1 $0x393138323733363435
	bufmerged.s tmsg1 "Globals:Str: Int[" gint1 "] Str[" gstr1 "] Buf[" gbuf1 "]\n"
	call PRINT_ME tmsg1
	bufmerged.b tmsg1 "Globals:Buf: Int[" gint1 "] Str[" gstr1 "] Buf[" gbuf1 "]\n"
	call PRINT_ME tmsg1
	# Global vars using explicit letglobal.type var setting
	letglobal.i gint2 202
	letglobal.s gstr2 "Global Vars Set2"
	letglobal.b gbuf2 $0x393831323733363435
	bufmerged.s tmsg2 "Globals:Str: Int[" gint2 "] Str[" gstr2 "] Buf[" gbuf2 "]\n"
	call PRINT_ME tmsg2
	bufmerged.b tmsg2 "Globals:Buf: Int[" gint2 "] Str[" gstr2 "] Buf[" gbuf2 "]\n"
	call PRINT_ME tmsg2
	# Global vars using implicit letglobal var setting
	letglobal gint3 203
	letglobal gstr3 "Global Vars Set3"
	letglobal gbuf3 $0x393837363132333435
	bufmerged.s tmsg3 "Globals:Str: Int[" gint3 "] Str[" gstr3 "] Buf[" gbuf3 "]\n"
	call PRINT_ME tmsg3
	bufmerged.b tmsg3 "Globals:Buf: Int[" gint3 "] Str[" gstr3 "] Buf[" gbuf3 "]\n"
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
	add intres int1 20
	bufmerged.s tmsg "\n\nArg1:" str1 "\n" "Arg2:Adjusted int1 is:" intres "\n\n"
	call PRINT_ME tmsg
	ret

!func CHECK_FARG_REF_LOCAL_NOGO
	letlocal lsrc "LOCAL_NOGO:1: FArg passed cant be a local var"
	call PRINT_ME lsrc
	letstr gsrc "LOCAL_NOGO:2: FArg passed can be a global var"
	call PRINT_ME gsrc
	ret

!func CHECK_FARGS
	letstr fargtest1 "This is a global var passed to func using func args"
	call CHECK_FARG_L1 fargtest1
	letstr fargtest2 "This is another global var passed to func using func args"
	call CHECK_FARG_L1 fargtest2
	letint testint1 100
	call CHECK_FARG_MULTI fargtest1 testint1
	call CHECK_FARG_REF_LOCAL_NOGO
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
	call VARIABLES
	call GLOBAL_VARS
	call CONDITIONS
	call CHECK_FARGS
	call LOCAL_VARS

