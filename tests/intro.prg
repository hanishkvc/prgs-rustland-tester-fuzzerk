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
!label	VARIABLES
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
!label CC_GOOD
	add CheckInt CheckInt 1
	ret

!label CC_BAD
	add CheckInt CheckInt 1
	ret

!label CC_CHECK
	ifge CheckInt 2 goto CCC_BAD
	letbuf tMsg "Yes IfLT seems fine\n"
	iobwrite term tMsg
	jump CCC_RET
!label CCC_BAD
	letbuf tMsg "No IfLT seems messed up\n"
	iobwrite term tMsg
!label CCC_RET
	ret

!label CONDITIONS
	letint CheckInt 0
	iflt 0 1 call CC_GOOD
	ifgt 0 1 call CC_BAD
	call CC_CHECK
	ret


	#
	# Initialise
	#
!label INIT
	iobnew term console
	ret


	#
	# The practical Program entry point
	#
!label StArT
	call INIT
	call VARIABLES
	call CONDITIONS

