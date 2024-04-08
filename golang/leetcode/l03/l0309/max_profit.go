package l0309

func maxProfit(prices []int) int {
	profit := make([]int, len(prices))

	result := 0
	for i := range prices {
		maxProfit := 0
		maxPreProfit := 0
		for j := 0; j < i; j++ {
			if j-2 >= 0 && maxPreProfit < profit[j-2] {
				maxPreProfit = profit[j-2]
			}

			if maxProfit < maxPreProfit+prices[i]-prices[j] {
				maxProfit = maxPreProfit + prices[i] - prices[j]
			}
		}
		profit[i] = maxProfit
		if result < maxProfit {
			result = maxProfit
		}
	}

	return result
}
