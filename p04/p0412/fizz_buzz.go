package p0412

import (
	"fmt"
)

func fizzBuzz(n int) []string {
	result := make([]string, n)
	for i := range result {
		if (i+1)%15 == 0 {
			result[i] = "FizzBuzz"
			continue
		}

		if (i+1)%3 == 0 {
			result[i] = "Fizz"
			continue
		}

		if (i+1)%5 == 0 {
			result[i] = "Buzz"
			continue
		}
		result[i] = fmt.Sprintf("%d", i+1)
	}
	return result
}
