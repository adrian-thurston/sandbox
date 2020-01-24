
all: fromgo/libflux.a
	go build -x

fromgo/libflux.a: rust/flux.o
	ar cr $@ $<
	echo "const char *uniq = \"`md5sum $@ | awk '{print$$1;}'`\";" > fromgo/uniq.c

rust/flux.o: rust/flux.c
	gcc -c -o $@ $<
