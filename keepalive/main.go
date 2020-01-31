package main

import (
	"syscall"
	"fmt"
	"runtime"
)

/*
 * This demonstrates the hazards of using finalizers, as described in the
 * SetFinalizer/KeepAlive documentation. The code below comes from the
 * KeepAlive docs.
 *
 * https://golang.org/pkg/runtime/#SetFinalizer
 * https://golang.org/pkg/runtime/#KeepAlive
 *
 * To demonstrate the problem, make, then start ./keepalive. Then from another
 * terminal echo some bytes into the fifo. You should see the finalizer close
 * the FD, then the read fail. Using KeepAlive below prevents this.
 */

type File struct { d int }

/* This takes the place of p.d in the original example. We want to induce the
 * garbage collector after pulling the int from the struct, but before the read
 * call happens. Using a function call for that. */
func get_fd( f *File ) int {
	/* Take the int from the struct. After this statement we are finished with
	 * the struct. */
	d := f.d;

	/* Induce the garbage collector. */
	steps := 96;
	for i := 0; i < steps; i++ {
		mem := make( [] byte, 1024*1024, 1024*1024 );
		mem[10] = 60;
	}

	/* Return the file descriptor. */
	return d;
}

func main() {
	d, err := syscall.Open( "fifo", syscall.O_RDONLY, 0 )

	if ( err != nil ) {
		fmt.Println( "open FAILED" );
	} else {
		fmt.Println( "fifo is open" );
		p := &File{d}
		runtime.SetFinalizer( p, func(p *File) {
			fmt.Println( "closing file from finalizer" );
			syscall.Close(p.d);
		})

		var buf [10]byte;
		n, err := syscall.Read( get_fd(p), buf[:] )
		if ( err != nil ) {
			fmt.Println( "read FAILED" )
		} else {
			fmt.Println( "read", n, "bytes:", buf )
		}

		// runtime.KeepAlive(p)
	}
}
