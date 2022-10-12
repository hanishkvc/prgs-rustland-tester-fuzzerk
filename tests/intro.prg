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

!func CHECK_FARGS
	letstr fargtest1 "This is a global var passed to func using func args"
	call CHECK_FARG_L1 fargtest1
	letstr fargtest2 "This is another global var passed to func using func args"
	call CHECK_FARG_L1 fargtest2
	letint testint1 100
	call CHECK_FARG_MULTI fargtest1 testint1
	ret


#
# Check function local vars
#
!func LOCAL_VARS_IN
	letlocal lint 123
	letlocal lstr "hello me local in"
	letlocal lbuf $0x39383736353433323130
	bufmerged.s tmsg "LocalsIN: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	bufmerged.b tmsg "LocalsIN: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	ret

!func LOCAL_VARS
	letlocal lint 565
	letlocal lstr "hello me local"
	letlocal lbuf $0x30313233343536373839
	call LOCAL_VARS_IN
	bufmerged.s tmsg "Locals: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
	bufmerged.b tmsg "Locals: Int[" lint "] Str[" lstr "] Buf[" lbuf "]\n"
	call PRINT_ME tmsg
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
	call CONDITIONS
	call CHECK_FARGS
	call LOCAL_VARS

