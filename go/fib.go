package main

import (
	"fmt"
	"os"
	"strconv"
)

func fib(x uint64) uint64 {
	if x <= 1 {
		return 1
	}
	return fib(x-1) + fib(x-2)
}

func main() {
	x, err := strconv.ParseUint(os.Args[1], 10, 64)
	if err != nil {
		panic(err)
	}
	fmt.Println(fib(x))
}
