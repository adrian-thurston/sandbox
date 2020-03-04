package FromGo

// #cgo LDFLAGS: -lflux -L ${SRCDIR}/
// #include "wrapper.h"
// #cgo pkg-config: cgo-rust
import "C"

func FluxWrapper() {
	C.flux_wrapper()
}

