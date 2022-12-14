#
# bench mark few ops
#
#

	jump START


#
# time related
#

!func time_start
	letint time1 __TIME__STAMP__
	ret


!func time_done sMsgFor
	letint		time2 __TIME__STAMP__
	sub		tDiff time2 time1
	bufmerged	sMsg "TDiff:" !str(tDiff) ":For:" sMsgFor "\n"
	iobwrite	term sMsg
	ret


#
# Bench Me helper
#

!func bench_me iTill
	letint		iEnd	iTill
	call		time_start
	letint		loopCnt	0
!label	bench_me_again
	inc		loopCnt
	# Call your logic here
	# ?????
	call		bench_add_once 1024
	iflt		loopCnt	iEnd goto bench_me_again
	bufmerged	bMsg "BenchMe, " !str(loopCnt)
	call		time_done bMsg
	ret


#
# Alu related
#

!func bench_add_once arg1
	add	iDest	arg1	1024
	ret


!func bench_incdec iTill
	letint		iEnd	iTill
	call		time_start
	letint		loopCnt	0
!label	bench_incdec_inc
	inc		loopCnt
	iflt		loopCnt	iEnd goto bench_incdec_inc
	bufmerged	bMsg "IncDecInc, " !str(loopCnt)
	call		time_done bMsg
	call		time_start
	letint		loopCnt	iTill
!label	bench_incdec_dec
	dec		loopCnt
	ifgt		loopCnt	0 goto bench_incdec_dec
	bufmerged	bMsg "IncDecDec, " !str(loopCnt)
	call		time_done bMsg
	ret


!func bench_alu_addg iTill
	letglobal.i	iEnd iTill
	call		time_start
	letint		loopCnt 0
!label addg_again
	add		iDest 1024 1024
	inc		loopCnt
	iflt		loopCnt iEnd goto addg_again
	bufmerged	sMsg "Alu AddG, " !str(loopCnt)
	call		time_done sMsg
	ret


!func bench_alu_addl iTill
	letlocal.i	iEnd iTill
	call		time_start
	letlocal.i	loopCnt 0
	letlocal.i	iDest 0
!label addl_again
	add		iDest 1024 1024
	inc		loopCnt
	iflt		loopCnt iEnd goto addl_again
	bufmerged	sMsg "Alu AddL, " !str(loopCnt)
	call		time_done sMsg
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

#
# Variables
#

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


!func bench_xopstr iTill
	call		time_start
	letint		loopCnt 0
!label xopstr_again
	bufmerged	btemp !str(loopCnt)
	inc		loopCnt
	iflt		loopCnt iTill goto xopstr_again
	bufmerged	bMsg "XOpStr, " !str(loopCnt)
	call		time_done bMsg
	ret


#
# IOB
#

!func bench_iob iTill
	letint		iEnd	iTill
	iobnew		fnull	filewriter:/dev/null
	call		time_start
	letint		loopCnt	0
	letbuf		bdata	$0x30313233343536373839
!label	iob_again
	inc		loopCnt
	iobwrite	fnull	bdata
	iflt		loopCnt	iEnd goto iob_again
	bufmerged	bMsg "IOB, " !str(loopCnt)
	call		time_done bMsg
	ret


#
# msleep
#

!func bench_msleep
	sleepmsec 10
	ret



!label START

	iobnew	term console
	call	bench_me 1024000
	call	bench_incdec 1024000
	call	bench_alu_addl 1024000
	call	bench_alu_addg 1024000
	call	bench_alu_mult 1024000
	call	bench_letstr 1024000
	call	bench_xopstr 1024000
	call	bench_iob 1024000

