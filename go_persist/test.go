package main

import (
	"fmt"
	"sync"
)

type OwningMutex[T any] struct {
	mutex sync.Mutex
	value T
}

func (om *OwningMutex[T]) Lock() *T {
	om.mutex.Lock()
	return &om.value
}

func (om *OwningMutex[T]) Unlock() {
	om.mutex.Unlock()
}

var MY_MUTEX = OwningMutex[int]{value: 42}

func foo(int_pointers *[]*int) {
	pointer := MY_MUTEX.Lock()
	defer MY_MUTEX.Unlock()
	*int_pointers = append(*int_pointers, pointer)
}

func main() {
	int_pointers := []*int{}
	foo(&int_pointers)
	foo(&int_pointers)
	foo(&int_pointers)
	fmt.Println(int_pointers)
}
