package main

import (
	"fmt"
	"sync"
)

func main() {
	var iterations int
	for {
		var count int
		var wg sync.WaitGroup

		increment := func() {
			for i := 0; i < 1000; i++ {
				count++
			}
			wg.Done()
		}

		wg.Add(2)
		go increment()
		go increment()

		wg.Wait()

		iterations++

		if count != 2000 {
			fmt.Printf("Data race detected on iteration %d! Final count: %d\n", iterations, count)
			break
		}
	}
}
