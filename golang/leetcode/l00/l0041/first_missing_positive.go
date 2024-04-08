package l0041

func firstMissingPositive(nums []int) int {
	length := len(nums)

	for i, v := range nums {
		for v > 0 && v <= length && v != i+1 {
			if v == nums[v-1] {
				v = 0
			} else {
				v, nums[v-1] = nums[v-1], v
			}
			nums[i] = v
		}
	}

	for i, v := range nums {
		if v != i+1 {
			return i + 1
		}
	}
	return length + 1
}
