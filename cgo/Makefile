
all: fromgo/libflux.a
	PKG_CONFIG=$(shell pwd)/cgo-pkg-config go build -x

fromgo/libflux.a: rust/flux.o
	ar cr $@ $<
	echo "const char *uniq = \"`shasum $@ | awk '{print$$1;}'`\";" > fromgo/uniq.c

rust/flux.o: rust/flux.c
	gcc -c -o $@ $<
