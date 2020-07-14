package p0704

const lineSearchPack = 3

func search(nums []int, target int) (result int) {
	// defer func() {
	// 	fmt.Println(nums, target, result)
	// }()
	n := len(nums)
	if n < lineSearchPack {
		for i, v := range nums {
			if v == target {
				return i
			}
		}
		return -1
	}
	mid := n / 2

	if nums[mid] <= target {
		r := search(nums[mid:], target)
		if r == -1 {
			return -1
		}
		return mid + r
	}

	return search(nums[:mid], target)
}
