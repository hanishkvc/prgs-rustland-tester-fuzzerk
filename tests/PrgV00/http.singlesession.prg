# Test HTTP, Use a single session across loop iterations
iob new
fc FC100
iob write
iob flush
sys sleep
loop inc
loop iflt 10 abspos 2

