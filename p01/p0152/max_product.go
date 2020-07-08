package p0152

func maxProduct(nums []int) int {
	n := len(nums)
	max := make([]int, n)
	min := make([]int, n)
	max[0] = nums[0]
	min[0] = nums[0]
	result := max[0]
	for i := 1; i < n; i++ {
		max[i], min[i] = cacualte(max[i-1]*nums[i], min[i-1]*nums[i], nums[i])
		if max[i] > result {
			result = max[i]
		}
	}
	return result
}

func cacualte(a, b, c int) (max, min int) {
	if a < b {
		a, b = b, a
	}
	if b < c {
		b, c = c, b
	}
	if a < b {
		a, b = b, a
	}

	return a, c
}
