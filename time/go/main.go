package main

import (
	"fmt"
	"time"
)

func main() {
	lit := "2018-11-29T09:00:00Z"
	t, err := time.Parse( time.RFC3339Nano, lit )
	if ( err != nil ) {
		fmt.Println( err )
	} else {
		fmt.Println( t )
	}
}
