# Test Buf operations
#

	jump START


!label TEST_BUFS
	letbuf msg1 "Test Bufs"
	bufsmerge msgStart msg1 msgnl
	iobwrite term msgStart
	bufnew tbuf1 16
	letint tb2size 32
	bufnew tbuf2 tb2size
	letbuf tb3size 32
	bufnew tbuf3 tb3size
	ret


!label INITS
	letbuf msgnl $0x0A
	iobnew term console
	ret


!label START
	call INITS
	call TEST_BUFS


