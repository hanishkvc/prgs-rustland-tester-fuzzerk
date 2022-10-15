#
# Test Buf operations
# Also test extra white spaces between operands
#

	jump START


# Expects bsize buffer to contain size of buffer created
!func PRINT_BCREATED asize sizeType
	letlocal.i  isize asize
	bufmerged.s bmsg "BufCreated: of size " asize " ie " isize ", sizeType " sizeType "\n"
	iobwrite    term bmsg
	ret


!func TEST_BUFS
	letbuf    msgStart "Test Bufs\n"
	iobwrite  term     msgStart

	bufnew    tbuf1 16
	call      PRINT_BCREATED 16 "literal"

	letint    tb2size 32
	bufnew    tbuf2   tb2size
	call      PRINT_BCREATED tb2size "int"

	letstr    tb4size 64
	bufnew    tbuf4   tb4size
	call      PRINT_BCREATED tb4size "str"

	letbuf    tb3size 128
	bufnew    tbuf3   tb3size
	call      PRINT_BCREATED tb3size "buf"
	ret


!func INITS
	iobnew term  console
	ret


!label START
	call INITS
	call TEST_BUFS

