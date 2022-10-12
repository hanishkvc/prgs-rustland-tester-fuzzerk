# Test if instruction

	jump START

!func PRINT_IFLT
	iobwrite term "IfLt Call Ok\n"
	ret


!func PRINT_IFGT
	iobwrite term "Ifgt Call Ok\n"
	ret


!func PRINT_IFEQINT
	iobwrite term "Ifeqint Call Ok\n"
	ret


!func PRINT_IFEQSTR
	iobwrite term "Ifeqstr Call Ok\n"
	ret


!func PRINT_NOTME
	iobwrite term "IfInvalidCondCall: Shouldnt see me\n"
	ret


!func COND_CALLS
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

