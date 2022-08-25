#
# Test Buf8Randomize
#
	letint loopcnt 0
	letbuf testbuf 0x30313233343536373839
	iobnew logme filewriter:/tmp/test.buf8randomize.bin create=yes
	letbuf newline 0x0A
	iobwrite logme testbuf
	iobwrite logme newline
!label repeat
	buf8randomize testbuf 1 -1 -1
	iobwrite logme testbuf
	iobwrite logme newline
	inc loopcnt
	checkjump loopcnt $10 repeat __NEXT__ __NEXT__
	iobclose logme

