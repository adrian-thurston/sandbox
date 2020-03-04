
GO=$(HOME)/devel/go/bin/go

all: wrapper/libmylib.a
	$(GO) build -a -work -k /tmp/gowork

wrapper/libmylib.a: mylib/mylib.o
	ar cr $@ $<

mylib/mylib.o: mylib/mylib.c
	gcc -c -o $@ $<
