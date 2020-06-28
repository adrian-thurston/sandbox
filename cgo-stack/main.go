package main

// #include "stacks.h"
import "C"

func routine( done chan bool ) {
	C.stacks()
	done <- true
}

func main() {
	done := make(chan bool)
	go routine(done)
	go routine(done)
	go routine(done)
	go routine(done)
	go routine(done)
	go routine(done)
	go routine(done)
	go routine(done)
	<-done
	<-done
	<-done
	<-done
	<-done
	<-done
	<-done
	<-done
}
