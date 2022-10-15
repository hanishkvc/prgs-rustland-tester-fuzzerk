#
# Test HTTP
#
# Start a new/seperate session for each loop iteration
# Save sent data as well as got response into a local file
#

	jump START

!func COMM_WITH_SERVER
	iobnew srv1 tlsclient:127.0.0.1:8088 domain=127.0.0.1 server_cert_check=no
	fcget FC100 bufFCGot
	#fcget FC_OkReqInBtw bufFCGot
	iobwrite srv1 bufFCGot
	iobflush srv1
	bufnew bufHttpGot 4096
	iobread srv1 bufHttpGot
	iobclose srv1
	ret

!func SAVE_TO_FILE
	bufmerged theMarker "\n\n\n\n**** NEW SET ****\n" __TIME__STAMP__ "\n*****************\n"
	iobwrite fsave theMarker
	iobwrite fsave bufFCGot
	iobwrite fsave markernl
	iobwrite fsave bufHttpGot
	ret

!func FILE_ID
	bufmerged fileid "FuzzerK:Save Of:HttpGetPrg:FileID:" __RANDOM__BYTES__16 "\n"
	iobwrite fsave fileid
	letbuf marker09 $0x0A0930313233343536373839090A
	buf8randomize marker09 3 2
	iobwrite fsave marker09
	iobwrite fsave markernl
	ret



!label START

	letint loopcnt 0
	iobnew fsave filewriter:/tmp/http.got.bin create=yes
	letbuf markernl $0x0A
	call FILE_ID

!label repeatagain

	call COMM_WITH_SERVER
	call SAVE_TO_FILE

	sleepmsec 1000

	inc loopcnt
	#checkjump loopcnt 10 repeatagain __NEXT__ __NEXT__
	iflt loopcnt 10 repeatagain

	iobclose fsave

