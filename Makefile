

cbuild:
	cargo build

ctests:
	cargo test -- --show-output

test_general:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/test02.fc --fc FC300

test_http:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --fc FC100 --loopcnt 4

dump_ascii:
	gcc -o misc/dump_ascii_printable misc/dump_ascii.c
	gcc -o misc/dump_ascii misc/dump_ascii.c

clean_misc:
	rm misc/dump_ascii_printable
	rm misc/dump_ascii

