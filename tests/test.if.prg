# Test if instruction

	jump START

!label PRINT_IFLT
	letbuf tmsg "IfLt Call Ok\n"
	iobwrite term tmsg
	ret


!label PRINT_IFGT
	letbuf tmsg "Ifgt Call Ok\n"
	iobwrite term tmsg
	ret


!label PRINT_IFEQINT
	letbuf tmsg "Ifeqint Call Ok\n"
	iobwrite term tmsg
	ret


!label PRINT_IFEQSTR
	letbuf tmsg "Ifeqstr Call Ok\n"
	iobwrite term tmsg
	ret


!label PRINT_NOTME
	letbuf tmsg "IfInvalidCondCall: Shouldnt see me\n"
	iobwrite term tmsg
	ret


!label COND_CALLS
	letint Int1		1
	iflt   0		1 	call PRINT_IFLT
	ifgt   Int1		0 	call PRINT_IFGT
	ifgt   Int1		1 	call PRINT_NOTME
	ifeq   10		10 	call PRINT_IFEQINT
	ifne   10		10 	call PRINT_NOTME
	letstr StrTest		"test me"
	ifeq   "test me"  	StrTest	call PRINT_IFEQSTR
	ifeq   "test me not"  	StrTest	call PRINT_NOTME
	ret


!label START
	iobnew term console
	letint CheckValue1 0
	letbuf msgSad "Msg:May not see me"

	call COND_CALLS

!label DUMP_MSG1
	add CheckValue1 CheckValue1 1
	letbuf.s CV1 CheckValue1
	bufmerged theMsg "Msg1: " CV1 $0x0A
	iobwrite term theMsg
	iflt CheckValue1 2 goto DUMP_MSG1

	ifeq "Msg1: 1\n" theMsg goto DUMP_MSG1

	ifeq "Msg1: 2\n" theMsg goto DUMP_MSG2

	iobwrite term msgSad

!label DUMP_MSG2
	bufmerged.b theMsg "Msg2: At end CheckValue1:" CV1 $0x0A
	iobwrite term theMsg
	#WONTWORK bcas binbuf $0x0A gets converted to hexstring by bufmerged.s
	#bufmerged.s theMsg "Msg2: At end CheckValue1:" CheckValue1 $0x0A
	bufmerged.s theMsg "Msg2: At end CheckValue1:" CheckValue1 "\n"
	iobwrite term theMsg

