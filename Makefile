

cbuild:
	cargo build

cclean:
	cargo clean

ctests:
	cargo test -- --show-output

test_general:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/test02.fc --fc FC300

test_general_loop:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/test02.fc --fc FC300 --loopcnt 10

test_http_console:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --fc FC100 --loopcnt 4

test_http_tcp:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --fc FC100 --loopcnt 4 --ioaddr tcpclient:127.0.0.1:8088

test_http_tls_seperate_cmdline:
	target/debug/fuzzerk --cfgfc tests/http01.fc --fc FC100 --ioaddr tlsclient:127.0.0.1:8088 --ioarg domain=127.0.0.1 --ioarg server_cert_check=no --loopcnt 10

test_http_tls_seperate_prgfile:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --prgfile tests/http.seperate.prg

test_http_tls_single_prgfile:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --prgfile tests/http.singlesession.prg

test_buf8randomize:
	target/debug/fuzzerk --prgfile tests/test.buf8randomize.prg

dump_ascii:
	gcc -o misc/dump_ascii_printable misc/dump_ascii.c
	gcc -o misc/dump_ascii misc/dump_ascii.c

pdf:
	rst2pdf README.rst README.pdf

clean_misc:
	rm misc/dump_ascii_printable || /bin/true
	rm misc/dump_ascii || /bin/true

clean_all: clean_misc cclean
	rm README.pdf || /bin/true

