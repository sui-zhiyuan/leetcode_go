package p5446

import (
	"sort"
)

func minDifference(nums []int) int {
	n := len(nums)
	if n <= 4 {
		return 0
	}

	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	min := nums[n-4] - nums[0]

	for i := 1; i <= 3; i++ {
		if min > nums[n-4+i]-nums[i] {
			min = nums[n-4+i] - nums[i]
		}
	}

	return min
}
