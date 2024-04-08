package l5460

func numIdenticalPairs(nums []int) int {
	result := 0

	for i := range nums {
		for j := 0; j < i; j++ {
			if nums[j] == nums[i] {
				result++
			}
		}
	}

	return result
}
