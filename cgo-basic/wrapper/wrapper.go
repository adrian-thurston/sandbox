package Wrapper

// #cgo LDFLAGS: -lmylib -L ${SRCDIR}/
// #include "wrapper.h"
import "C"

func Wrapper() {
	C.wrapper()
}

