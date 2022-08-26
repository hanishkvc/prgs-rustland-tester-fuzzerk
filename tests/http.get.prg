#
# Test HTTP
#
# Start a new/seperate session for each loop iteration
# Save sent data as well as got response into a local file
#

	jump START

!label COMM_WITH_SERVER
	iobnew srv1 tlsclient:127.0.0.1:8088 domain=127.0.0.1 server_cert_check=no
	fcget FC100 bufFCGot
	#fcget FC_OkReqInBtw bufFCGot
	iobwrite srv1 bufFCGot
	iobflush srv1
	bufnew bufHttpGot 4096
	iobread srv1 bufHttpGot
	iobclose srv1
	ret

!label SAVE_TO_FILE
	iobwrite fsave markerstart
	letbuf ts __TIME__STAMP__
	iobwrite fsave ts
	iobwrite fsave markernl
	iobwrite fsave markerend
	iobwrite fsave bufFCGot
	iobwrite fsave markernl
	iobwrite fsave bufHttpGot
	ret

!label FILE_ID
	letbuf sfileid FuzzerK:Save Of:HttpGetPrg:FileID:
	letbuf fileid __RANDOM__BYTES__16
	iobwrite fsave sfileid
	iobwrite fsave fileid
	iobwrite fsave markernl
	letbuf marker09 0x0A0930313233343536373839090A
	buf8randomize marker09 3 2
	iobwrite fsave marker09
	iobwrite fsave markernl
	ret



!label START

	letint loopcnt 0
	iobnew fsave filewriter:/tmp/http.got.bin create=yes
	letbuf marker01 **** NEW SET ****
	letbuf marker02 *****************
	letbuf markernl 0x0A
	bufsmerge markerstart markernl markernl markernl marker01 markernl
	bufsmerge markerend markernl marker02 markernl
	call FILE_ID

!label repeatagain

	call COMM_WITH_SERVER
	call SAVE_TO_FILE

	sleepmsec 1000

	inc loopcnt
	checkjump loopcnt $10 repeatagain __NEXT__ __NEXT__

	iobclose fsave

