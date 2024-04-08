package l0007

import (
	"math"
)

func reverse(x int) int {
	neg := x < 0

	var c int64 = int64(x)
	if neg {
		c = -c
	}

	var result int64

	for c > 0 {
		x := c % 10
		c = c / 10
		result = result * 10
		result = result + x
	}

	if neg {
		result = -result
	}

	if result > math.MaxInt32 || result < math.MinInt32 {
		return 0
	}

	return int(result)
}
