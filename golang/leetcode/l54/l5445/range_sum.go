package l5445

import (
	"sort"
)

const mod = 1_000_000_007

func rangeSum(nums []int, n int, left int, right int) int {
	sums := make([]int, 0, n*(n+1)/2)
	left = left - 1
	right = right - 1
	for i := 0; i < n; i++ {
		sum := 0
		for j := i; j < n; j++ {
			sum = sum + nums[j]
			sums = append(sums, sum)
		}
	}

	sort.Slice(sums, func(i, j int) bool {
		return sums[i] < sums[j]
	})

	result := 0
	for k := left; k <= right; k++ {
		result = (result + sums[k]) % mod
	}
	return result
}
