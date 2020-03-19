
roundtrip: main.c rust1 rust2 Makefile
	g++ -o $@ $< \
		-Lrust1/target/debug -Wl,-rpath,$(PWD)/rust1/target/debug \
		-Lrust2/target/debug -Wl,-rpath,$(PWD)/rust2/target/debug \
		-lrust1 -lrust2 -lpthread 

#	g++ -o $@ $< rust/target/debug/librust.a -lpthread -ldl

.PHONY: rust1 rust2

rust1:
	cd $@ && cargo build

rust2:
	cd $@ && cargo build
