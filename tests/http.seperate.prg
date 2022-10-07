# Test HTTP, Start a new/seperate session for each loop iteration
	letint loopcnt $0
!label repeatagain
	iobnew srv1 tlsclient:127.0.0.1:8088 domain=127.0.0.1 server_cert_check=no
	fcget FC100 fc100Buf
	iobwrite srv1 fc100Buf
	iobflush srv1
	bufnew rbuf 1024
	iobread srv1 rbuf
	iobclose srv1
	sleepmsec $1000
	inc loopcnt
	iflt $10 loopcnt goto repeatagain

