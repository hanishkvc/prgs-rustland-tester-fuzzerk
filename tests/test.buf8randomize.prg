#
# Test Buf8Randomize
#
	letint loopcnt 0
	letbuf testbuf $0x30313233343536373839
	iobnew logme filewriter:/tmp/test.buf8randomize.bin create=yes
	letbuf newline $0x0A
	iobwrite logme testbuf
	iobwrite logme newline
	letint randcnt 2
!label repeat
	#buf8randomize testbuf 1 -1 -1
	#buf8randomize testbuf 2 -1 -1
	#buf8randomize testbuf 2 5 -1
	#buf8randomize testbuf 2 0 5
	buf8randomize testbuf randcnt 2 7 0xf830 0x39
	iobwrite logme testbuf
	iobwrite logme newline
	inc loopcnt
	iflt loopcnt 10 repeat
	iobclose logme

