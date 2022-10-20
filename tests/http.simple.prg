# Test HTTP access to google, Use a single session across loop iterations

	letint loopcnt 0
	#iobnew srv1 tcpclient:google.com:80
	iobnew srv1 tlsclient:google.com:443 domain=google.com
	#iobnew srv1 tlsclient:127.0.0.1:8088 domain=127.0.0.1 server_cert_check=no
	iobnew file1 filewriter:/tmp/http.simple.log create=yes
	iobnew term console

	mult bsize 4096 4096

	letint time1 __TIME__STAMP__

!label repeatagain

	fcget FC100 fc100Buf
	letstr sloopcnt loopcnt
	bufmerged.b bmsg "Cnt:" sloopcnt ":Req:" fc100Buf "\n"
	iobwrite term bmsg
	iobwrite srv1 fc100Buf
	iobflush srv1

	bufnew readbuf bsize
	iobread srv1 readbuf
	#emagic 0x010 readbuf
	iobwrite file1 readbuf
	bufmerged.b bmsg "Cnt:" sloopcnt ":Res:" readbuf "\n"
	iobwrite term bmsg

	#sleepmsec 10
	sleepmsec 1000
	inc loopcnt
	iflt loopcnt 10 goto repeatagain

	letint time2 __TIME__STAMP__
	sub tdiff time2 time1
	letstr stdiff tdiff
	bufmerged smsg "TDiff:" stdiff ":Cnt:" sloopcnt "\n"
	iobwrite term smsg

