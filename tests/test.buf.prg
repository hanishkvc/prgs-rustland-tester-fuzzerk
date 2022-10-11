#
# Test Buf operations
# Also test extra white spaces between operands
#

	jump START


# Expects bsize buffer to contain size of buffer created
!label PRINT_BCREATED
	bufmerged.s bmsg "BufCreated:" bsize "\n"
	iobwrite    term bmsg
	ret


!label TEST_BUFS
	letbuf    msgStart "Test Bufs\n"
	iobwrite  term     msgStart

	bufnew    tbuf1 16
	letbuf.s  bsize 16
	call      PRINT_BCREATED

	letint    tb2size 32
	bufnew    tbuf2   tb2size
	letbuf.s  bsize   tb2size
	call      PRINT_BCREATED

	letstr    tb4size 64
	bufnew    tbuf4   tb4size
	letbuf.s  bsize   tb4size
	call      PRINT_BCREATED

	letbuf    tb3size 128
	bufnew    tbuf3   tb3size
	letbuf.s  bsize   tb3size
	call      PRINT_BCREATED
	ret


!label INITS
	iobnew term  console
	ret


!label START
	call INITS
	call TEST_BUFS

