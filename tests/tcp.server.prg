# Simulate a TCP Server, Use a single session across loop iterations
# Use tests/tcpserver.fc

	letint loopcnt 0
	iobnew srv1 tcpserver:127.0.0.1:8088
	iobnew file1 filewriter:/tmp/tcpserver.log

!label repeatagain

	bufnew ReadBuf 16
	emagic 0x010 ReadBuf
	iobread srv1 ReadBuf
	bufmerged.s sloopcnt loopcnt
	bufmerged WriteBuf "Cnt:" sloopcnt ":Read[" ReadBuf "]\n"
	iobwrite file1 WriteBuf

	fcget FC100 fc100Buf
	iobwrite srv1 fc100Buf
	iobflush srv1

	#sleepmsec 10
	sleepmsec 1000
	inc loopcnt
	iflt loopcnt 10 goto repeatagain

