

cbuild:
	cargo build

ctests:
	cargo test -- --show-output

run_test:
	RUST_BACKTRACE=1 target/debug/fuzzerk tests/test02.fc FC300

dump_ascii:
	gcc -o misc/dump_ascii_printable misc/dump_ascii.c
	gcc -o misc/dump_ascii misc/dump_ascii.c

clean_misc:
	rm misc/dump_ascii_printable
	rm misc/dump_ascii

