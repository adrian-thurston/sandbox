
all: fromgo/libflux.a
	PKG_CONFIG=$(shell pwd)/cgo-pkg-config go build -x

fromgo/libflux.a: rust/flux.o
	ar cr $@ $<

rust/flux.o: rust/flux.c
	gcc -c -o $@ $<
