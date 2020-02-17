
value-ref.diff: main-value.txt main-ref.txt
	diff -u main-value.txt main-ref.txt > $@ || true

main-value.txt: main-value.o
	objdump -D $< > $@

main-ref.txt: main-ref.o
	objdump -D $< > $@

main-value.o: main-value.rs Makefile
	cp $< main.rs
	rustc --emit obj -C opt-level=3 main.rs
	mv main.o $@

main-ref.o: main-ref.rs Makefile
	cp $< main.rs
	rustc --emit obj -C opt-level=3 main.rs
	mv main.o $@
