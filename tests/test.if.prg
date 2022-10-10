# Test if instruction

	jump START


!label START
	iobnew term console
	letint CheckValue1 0
	letbuf msg1 "Msg1: "
	letbuf msg2 "Msg2:"
	letbuf msgSad "Msg:May not see me"

!label DUMP_MSG1
	add CheckValue1 CheckValue1 1
	letbuf.s CV1 CheckValue1
	bufsmerge theMsg msg1 CV1
	iobwrite term theMsg
	iflt.i CheckValue1 2 goto DUMP_MSG1

	ifeq.s "Msg1: 1" theMsg goto DUMP_MSG1

	ifeq.s "Msg1: 2" theMsg goto DUMP_MSG2

	iobwrite term msgSad

!label DUMP_MSG2
	iobwrite term msg2

