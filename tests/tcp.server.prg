# Simulate a TCP Server, Use a single session across loop iterations
# Use tests/tcpserver.fc

	letint loopcnt 0
	iobnew srv1 tcpserver:127.0.0.1:8088
	iobnew file1 filewriter:/tmp/tcpserver.log
	bufnew ReadBuf 16 

!label repeatagain

	iobread srv1 ReadBuf
	bufmerged WriteBuf "Cnt:" loopcnt ":Read[" ReadBuf "]\n"
	iobwrite file1 WriteBuf

	fcget FC100 fc100Buf
	iobwrite srv1 fc100Buf
	iobflush srv1

	#sleepmsec 10
	sleepmsec 1000
	inc loopcnt
	iflt loopcnt 10 goto repeatagain

