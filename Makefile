

cbuild:
	cargo build

ctests:
	cargo test -- --show-output

test_general:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/test02.fc --fc FC300

test_http_console:
	RUST_BACKTRACE=1 target/debug/fuzzerk --cfgfc tests/http01.fc --fc FC100 --loopcnt 4

test_http_tcp:
	target/debug/fuzzerk --cfgfc tests/http01.fc --fc FC100 --ioaddr tcpclient:127.0.0.1:8088 --loopcnt 4

test_http_tls_old:
	target/debug/fuzzerk --cfgfc tests/http01.fc --fc FC100 --ioaddr tlsclient:127.0.0.1:8088 --ioarg domain=127.0.0.1 --ioarg server_cert_check=no --loopcnt 100

test_http_tls_seperate:
	target/debug/fuzzerk --cfgfc tests/http01.fc --prgfile tests/http.seperate.prg --ioaddr tlsclient:127.0.0.1:8088 --ioarg domain=127.0.0.1 --ioarg server_cert_check=no

test_http_tls_single:
	target/debug/fuzzerk --cfgfc tests/http01.fc --prgfile tests/http.singlesession.prg --ioaddr tlsclient:127.0.0.1:8088 --ioarg domain=127.0.0.1 --ioarg server_cert_check=no

dump_ascii:
	gcc -o misc/dump_ascii_printable misc/dump_ascii.c
	gcc -o misc/dump_ascii misc/dump_ascii.c

clean_misc:
	rm misc/dump_ascii_printable
	rm misc/dump_ascii

