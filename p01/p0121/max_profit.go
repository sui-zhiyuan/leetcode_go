package p0121

func maxProfit(prices []int) int {
	if len(prices) <= 1 {
		return 0
	}
	min := prices[0]
	result := 0
	for _, v := range prices {
		if v-min > result {
			result = v - min
		}

		if v < min {
			min = v
		}
	}
	return result
}
