

cbuild:
	cargo build

ctests:
	cargo test -- --show-output

dump_ascii:
	gcc -o misc/dump_ascii_printable misc/dump_ascii.c
	gcc -o misc/dump_ascii misc/dump_ascii.c

clean_misc:
	rm misc/gen_printable_ascii

