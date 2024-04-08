package l0561

import (
	"sort"
)

func arrayPairSum(nums []int) int {
	n := len(nums) / 2
	sort.Slice(nums, func(i, j int) bool { return nums[i] < nums[j] })

	result := 0
	for i := 0; i < n; i++ {
		result += nums[i*2]
	}
	return result
}
