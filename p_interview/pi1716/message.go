package pi1716

func massage(nums []int) int {
	nums = append(nums, 0, 0)
	arr := make([]int, len(nums))
	for i, v := range nums {
		max := 0
		if i-3 >= 0 && max < arr[i-3] {
			max = arr[i-3]
		}
		if i-2 >= 0 && max < arr[i-2] {
			max = arr[i-2]
		}

		arr[i] = max + v
	}
	return arr[len(arr)-1]
}
