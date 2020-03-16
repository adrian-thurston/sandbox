package Bridge

// #cgo LDFLAGS: -lrust -L${SRCDIR}/../rust/target/debug
// #include "rust.h"
import "C"

func Bridge() {
	println("hello from the bridge");
	C.hello_from_rust();
	println("back in the bridge");
}

