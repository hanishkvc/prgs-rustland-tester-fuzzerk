#
# Test HTTP access to a site
# Use a single session to send multiple requests
#
# Notes:
# * currently below it tries to fetch from google
# * this logic blindly keeps sending new requests, even if
#   prev requests full data is not yet fetched.
#   * so best to fetch urls which fit within a single tcp packet
#     and inturn a single read request.
# * pass http.simple.fc config file using --cfgfc
#

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

!label ending
	letint time2 __TIME__STAMP__
	sub tdiff time2 time1
	letstr stdiff tdiff
	bufmerged smsg "TDiff:" stdiff ":Cnt:" sloopcnt "\n"
	iobwrite term smsg

