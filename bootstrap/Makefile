all: driver-static driver-dynamic

driver-static: main.c rust 
	gcc -o $@ $< \
		target/debug/libflux.a \
		-lpthread -ldl

driver-dynamic: main.c rust 
	gcc -o $@ $< \
		-lflux -Ltarget/debug \
		-Wl,-rpath,$(PWD)/target/debug

.PHONY: rust

rust:
	cargo build
