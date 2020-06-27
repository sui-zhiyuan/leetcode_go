package p5434

func longestSubarray(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	sum := []int{}

	count := 0
	for _, v := range nums {
		if v == 0 {
			sum = append(sum, count)
			count = 0
			continue
		}

		count++
	}
	sum = append(sum, count)

	// fmt.Println(sum)
	if len(sum) == 1 {
		return sum[0] - 1
	}

	max := 0
	for i := 0; i < len(sum)-1; i++ {
		if sum[i]+sum[i+1] > max {
			max = sum[i] + sum[i+1]
		}
	}

	return max
}
