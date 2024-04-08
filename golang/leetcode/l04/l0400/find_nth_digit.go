package l0400

import (
	"fmt"
)

func findNthDigit(n int) int {
	for i := 1; i < 11; i++ {
		pow := 1
		for j := 1; j < i; j++ {
			pow *= 10
		}

		count := pow * 9 * i
		if n > count {
			n -= count
			fmt.Println("sub", i, count, n)
			continue
		}

		number := pow - 1 + (n-1)/i + 1
		los := i - (n-1)%i
		fmt.Println("find", number, los)

		for j := 1; j < los; j++ {
			number /= 10
		}

		return number % 10
	}

	return -1
}
