#
# Test Buf operations
#

	jump START


# Expects bsize buffer to contain size of buffer created
!label PRINT_BCREATED
	letbuf msgBufCreated "BufCreated:"
	bufsmerge bmsg msgBufCreated bsize msgnl
	iobwrite term bmsg
	ret


!label TEST_BUFS
	letbuf msg1 "Test Bufs"
	bufsmerge msgStart msg1 msgnl
	iobwrite term msgStart

	bufnew tbuf1 16
	letbuf.s bsize 16
	call PRINT_BCREATED

	letint tb2size 32
	bufnew tbuf2 tb2size
	letbuf.s bsize tb2size
	call PRINT_BCREATED

	letstr tb4size 64
	bufnew tbuf4 tb4size
	letbuf.s bsize tb4size
	call PRINT_BCREATED

	letbuf tb3size 128
	bufnew tbuf3 tb3size
	letbuf.s bsize tb3size
	call PRINT_BCREATED
	ret


!label INITS
	letbuf msgnl $0x0A
	iobnew term console
	ret


!label START
	call INITS
	call TEST_BUFS

