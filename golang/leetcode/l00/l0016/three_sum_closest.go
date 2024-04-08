package l0016

func threeSumClosest(nums []int, target int) int {
	quickSort(nums)
	min, set := 0, false
	for i, v := range nums {
		if i > len(nums)-3 {
			break
		}
		newVal := twoSumClosest(nums[i+1:], target-v)
		if !set || abs(min) > abs(newVal) {
			min = newVal
			set = true
		}
	}
	return target + min
}

func twoSumClosest(nums []int, target int) int {
	i, j := 0, len(nums)-1
	min := nums[i] + nums[j] - target
	for {
		if i == j {
			return min
		}
		newVal := nums[i] + nums[j] - target
		if abs(newVal) < abs(min) {
			min = newVal
		}
		switch {
		case nums[i]+nums[j] > target:
			j--
		case nums[i]+nums[j] < target:
			i++
		case nums[i]+nums[j] == target:
			return 0
		}
	}
}

func quickSort(nums []int) {
	z := nums[(0+len(nums)-1)/2]
	i, j := 0, len(nums)-1
	for i <= j {
		for nums[i] < z {
			i++
		}

		for nums[j] > z {
			j--
		}
		if i <= j {
			nums[i], nums[j] = nums[j], nums[i]
			i++
			j--
		}
	}
	if j > 0 {
		quickSort(nums[0 : j+1])
	}
	if i < len(nums)-1 {
		quickSort(nums[i:])
	}
}

func abs(a int) int {
	if a >= 0 {
		return a
	}
	return -a
}
