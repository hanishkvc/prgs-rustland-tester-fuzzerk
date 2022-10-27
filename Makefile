

cbuild:
	cargo build

cbuildr:
	cargo build --release

cclean:
	cargo clean

ctests:
	cargo test -- --show-output

run_prg_intro: cbuildr
	target/release/fuzzerk --asmfile tests/intro.prg

test_prg_intro: cbuild
	target/debug/fuzzerk --asmfile tests/intro.prg --blogdebug true

run_prg_bench:
	target/release/fuzzerk --asmfile tests/bench.prg

test_prg_bench:
	target/debug/fuzzerk --asmfile tests/bench.prg

run_prg_httpfetch:
	target/release/fuzzerk --cfgfc tests/http.simple.fc --asmfile tests/http.fetchfile.prg

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

test_http_tls_seperate_asmfile:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --asmfile tests/http.seperate.prg

test_http_tls_single_asmfile:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --asmfile tests/http.singlesession.prg

test_buf8randomize:
	target/debug/fuzzerk --asmfile tests/test.buf8randomize.prg

test_client_using_tcpserver_cmdline:
	target/debug/fuzzerk --cfgfc tests/test02.fc --fc FC100 --ioaddr tcpserver:127.0.0.1:8888 --loopcnt 1

test_client_using_tcpserver_asmfile:
	target/debug/fuzzerk --cfgfc tests/tcpserver.fc --asmfile tests/tcp.server.prg

dump_ascii:
	gcc -o misc/dump_ascii_printable misc/dump_ascii.c
	gcc -o misc/dump_ascii misc/dump_ascii.c

pdf:
	rst2pdf README.rst README.pdf

html:
	cmark --to html README.md > README.html

clean_misc:
	rm misc/dump_ascii_printable || /bin/true
	rm misc/dump_ascii || /bin/true

clean_doc:
	rm README.pdf README.html || /bin/true

clean_all: clean_misc cclean clean_doc

