# Test HTTP access to google, Use a single session across loop iterations

	letint loopcnt 0
	iobnew srv1 tcpclient:google.com:80
	iobnew file1 filewriter:/tmp/http.simple.log create=yes
	iobnew term console

	letint size1 4096
	mult bsize size1 size1

!label repeatagain

	fcget FC100 fc100Buf
	letstr sloopcnt loopcnt
	bufmerged.b bmsg "Cnt:" sloopcnt ":Req:" fc100Buf "\n"
	iobwrite term bmsg
	iobwrite srv1 fc100Buf
	iobflush srv1

	bufnew readbuf bsize
	iobread srv1 readbuf
	iobwrite file1 readbuf
	bufmerged.b bmsg "Cnt:" sloopcnt ":Res:" readbuf "\n"
	iobwrite term bmsg

	#sleepmsec 10
	sleepmsec 1000
	inc loopcnt
	iflt loopcnt 10 goto repeatagain

