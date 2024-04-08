package l5453

func getLastMoment(n int, left []int, right []int) int {
	ans := 0
	for _, v := range left {
		if v > ans {
			ans = v
		}
	}

	for _, v := range right {
		if n-v > ans {
			ans = n - v
		}
	}

	return ans
}
