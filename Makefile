

cbuild:
	cargo build

ctests:
	cargo test -- --show-output

genprintable:
	gcc -o misc/gen_printable_ascii misc/gen_printable_ascii.c

clean_misc:
	rm misc/gen_printable_ascii

