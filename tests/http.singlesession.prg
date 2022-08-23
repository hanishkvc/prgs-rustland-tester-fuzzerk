# Test HTTP, Use a single session across loop iterations
	letint loopcnt 0
	iobnew srv1 tlsclient:127.0.0.1:8088 domain=127.0.0.1 server_cert_check=no
!label repeatagain
	fcget FC100 fc100Buf
	iobwrite srv1 fc100Buf
	iobflush srv1
	#sleepmsec 10
	sleepmsec 1000
	inc loopcnt
	iflt 10 loopcnt goto repeatagain

