package FromGo

// #cgo LDFLAGS: -lflux -L ${SRCDIR}/
// #include "wrapper.h"
import "C"

func FluxWrapper() {
	C.flux_wrapper()
}

