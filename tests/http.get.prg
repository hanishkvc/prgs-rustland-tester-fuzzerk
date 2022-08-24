#
# Test HTTP
#
# Start a new/seperate session for each loop iteration
# Save sent data as well as got response into a local file
#
	letint loopcnt 0
	iobnew fsave filewriter:/tmp/http.got.bin append=no
	letbuf sfileid FuzzerK:Save Of:HttpGetPrg:FileID:
	letbuf fileid __RANDOM__BYTES__16
	letbuf marker01 **** NEW SET ****
	letbuf marker02 *****************
	letbuf markernl 0x0A
	letbuf marker09 0x0A0930313233343536373839090A
	iobwrite fsave sfileid
	iobwrite fsave fileid
	iobwrite fsave markernl
!label repeatagain
	iobnew srv1 tlsclient:127.0.0.1:8088 domain=127.0.0.1 server_cert_check=no
	#fcget FC100 fc100Buf
	fcget FC_OkReqInBtw fc100Buf
	iobwrite srv1 fc100Buf
	iobflush srv1
	bufnew httpgot 4096
	iobread srv1 httpgot
	iobclose srv1
	iobwrite fsave markernl
	iobwrite fsave markernl
	iobwrite fsave markernl
	iobwrite fsave marker01
	iobwrite fsave markernl
	letbuf ts __TIME__STAMP__
	iobwrite fsave ts
	iobwrite fsave markernl
	iobwrite fsave marker09
	iobwrite fsave markernl
	iobwrite fsave marker02
	iobwrite fsave markernl
	iobwrite fsave fc100Buf
	iobwrite fsave markernl
	iobwrite fsave httpgot
	sleepmsec 1000
	inc loopcnt
	checkjump loopcnt $10 repeatagain __NEXT__ __NEXT__
	iobclose fsave

