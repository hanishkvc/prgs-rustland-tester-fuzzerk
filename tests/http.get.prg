# Test HTTP, Start a new/seperate session for each loop iteration
	letint loopcnt 0
	iobnew fsave filewriter:/tmp/http.got.bin append=no
	letbuf marker01 **** NEW SET ****
	letbuf marker02 *****************
	letbuf markernl 0x0A
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
	iobwrite fsave marker02
	iobwrite fsave markernl
	iobwrite fsave fc100Buf
	iobwrite fsave httpgot
	sleepmsec 1000
	inc loopcnt
	iflt 10 loopcnt goto repeatagain
	iobclose fsave

