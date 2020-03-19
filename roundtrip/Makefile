
roundtrip: main.cc rust Makefile
	g++ -o $@ $< -Lrust/target/debug -Wl,-rpath,$(PWD)/rust/target/debug -lrust -lpthread 

#	g++ -o $@ $< rust/target/debug/librust.a -lpthread -ldl

.PHONY: rust

rust:
	cd rust && cargo build
