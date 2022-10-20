#
# Fetch the specified url from the specified site
# * specify site using iobnew here
# * specify the url in http.simple.fc and inturn specify fc file using --cfgfc
#   * if reqd one could specify the url directly here
#

	letint loopcnt 0
	#iobnew srv1 tcpclient:google.com:80
	iobnew srv1 tlsclient:google.com:443 domain=google.com read_timeout=1000
	#iobnew srv1 tlsclient:127.0.0.1:8088 domain=127.0.0.1 server_cert_check=no
	iobnew file1 filewriter:/tmp/http.simple.log create=yes
	iobnew term console

	mult bsize 4096 4096
	letint time1 __TIME__STAMP__

	# send request
	fcget FC100 fc100Buf
	iobwrite srv1 fc100Buf
	iobflush srv1
	bufmerged.b bmsg "Req:" fc100Buf "\n"
	iobwrite term bmsg

	# fetch data

!label fetchremaining

	bufnew readbuf bsize
	iobread srv1 readbuf
	getsize readbuf rbLen
	iobwrite file1 readbuf

	letstr sRbLen rbLen
	letstr sloopcnt loopcnt
	#bufmerged.b bmsg "Cnt:" sloopcnt ":Res:Size:" sRbLen ":Data:" readbuf "\n"
	bufmerged.b bmsg "Cnt:" sloopcnt ":Res:Size:" sRbLen "\n"
	iobwrite term bmsg

	inc loopcnt
	ifgt loopcnt 1024 goto breakout
	ifgt rbLen 0 goto fetchremaining

!label breakout
	letint time2 __TIME__STAMP__
	sub tdiff time2 time1
	letstr stdiff tdiff
	bufmerged smsg "TDiff:" stdiff ":Cnt:" sloopcnt "\n"
	iobwrite term smsg

