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
	bufmerged	sMsg "TDiff:" !str(tDiff) ":For:" sMsgFor "\n"
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
	bufmerged sMsg "Alu Add, " !str(loopCnt)
	call	time_done sMsg
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
	bufmerged sMsg "Alu Mult, " !str(loopCnt)
	call	time_done sMsg
	ret

!func bench_letstr iTill
	call		time_start
	letint		loopCnt 0
!label letstr_again
	letstr		sloopCnt loopCnt
	bufmerged	btemp sloopCnt
	inc		loopCnt
	iflt		loopCnt iTill goto letstr_again
	bufmerged	bMsg "LetStr, " !str(loopCnt)
	call		time_done bMsg
	ret

!func bench_xcaststr iTill
	call		time_start
	letint		loopCnt 0
!label xcaststr_again
	bufmerged	btemp !str(loopCnt)
	inc		loopCnt
	iflt		loopCnt iTill goto xcaststr_again
	bufmerged	bMsg "XCastStr, " !str(loopCnt)
	call		time_done bMsg
	ret


!label START

	iobnew	term console
	call	bench_alu_add 1024000
	call	bench_alu_mult 1024000
	call	bench_letstr 4024000
	call	bench_xcaststr 4024000

