# Test if instruction

	jump START


!label START
	iobnew term console
	letint CheckValue1 0
	letbuf msg1 "Msg1: "
	letbuf msg2 "Msg2:"
	letbuf msgSad "Msg:May not see me"
	letbuf msgNL $0x0A

!label DUMP_MSG1
	add CheckValue1 CheckValue1 1
	letbuf.s CV1 CheckValue1
	bufsmerge theMsg msg1 CV1 msgNL
	iobwrite term theMsg
	iflt CheckValue1 2 goto DUMP_MSG1

	ifeq "Msg1: 1\n" theMsg goto DUMP_MSG1

	ifeq "Msg1: 2\n" theMsg goto DUMP_MSG2

	iobwrite term msgSad

!label DUMP_MSG2
	iobwrite term msg2

