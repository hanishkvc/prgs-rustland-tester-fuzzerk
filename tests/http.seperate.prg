# Test HTTP, start a new session each time through the loop
iob new
fc FC100
iob write
iob flush
sys sleep
iob close
loop inc
loop iflt 10 abspos 1

