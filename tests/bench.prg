#
# bench mark few ops
#
#

	jump START

!func time_start
	letint time1 __TIME__STAMP__
	ret

!func time_done sMsgFor
	letint		time2 __TIME__STAMP__
	sub		tDiff time2 time1
	letstr		stDiff tDiff
	bufmerged	sMsg "TDiff:" stDiff ":For:" sMsgFor "\n"
	iobwrite	term sMsg
	ret

!func bench_alu_add iTill
	letlocal.i iEnd iTill
	call	time_start
	letint	loopCnt 0
!label add_again
	add	iDest 1024 1024
	inc	loopCnt
	iflt	loopCnt iEnd goto add_again
	call	time_done "Alu Add"
	ret

!func bench_alu_mult iTill
	call	time_start
	letint	loopCnt 0
!label mult_again
	mult	iDest 1024 1024
	mult	iDest 1024 1024
	mult	iDest 1024 1024
	mult	iDest 1024 1024
	mult	iDest 1024 1024
	mult	iDest 1024 1024
	mult	iDest 1024 1024
	mult	iDest 1024 1024
	add	loopCnt loopCnt 8
	iflt	loopCnt iTill goto mult_again
	call	time_done "Alu Mult"
	ret


!label START

	iobnew	term console
	call	bench_alu_add 1024000
	call	bench_alu_mult 1024000

