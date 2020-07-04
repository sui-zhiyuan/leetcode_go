package p0169

func majorityElement(nums []int) int {
	curr := 0
	count := 0

	for _, v := range nums {
		if count == 0 {
			curr = v
			count = 1
			continue
		}

		if curr == v {
			count++
			continue
		}
		count--
	}

	return curr
}
