#
# A intro program
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
	# Check out conditions
	#
!label LT_GOOD
	add CheckInt CheckInt 1
	ret

!label LT_BAD
	add CheckInt CheckInt 1
	ret

!label LT_CHECK
	ifge CheckInt 2 goto LTC_BAD
	letbuf tMsg "Yes IfLT seems fine\n"
	iobwrite term tMsg
	jump LTC_RET
!label LTC_BAD
	letbuf tMsg "No IfLT seems messed up\n"
	iobwrite term tMsg
!label LTC_RET
	ret

!label CONDITIONS
	letint CheckInt 0
	iflt 0 1 call LT_GOOD
	iflt 1 0 call LT_BAD
	call LT_CHECK
	ret


	#
	# Initialise
	#
!label INIT
	iobnew term console
	ret

!label StArT
	call INIT
	call VARIABLES
	call CONDITIONS

