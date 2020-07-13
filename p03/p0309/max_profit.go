package p0309

func maxProfit(prices []int) int {
	profit := make([]int, len(prices))

	result := 0
	for i := range prices {
		maxProfit := 0
		maxPreprofit := 0
		for j := 0; j < i; j++ {
			if j-2 >= 0 && maxPreprofit < profit[j-2] {
				maxPreprofit = profit[j-2]
			}

			if maxProfit < maxPreprofit+prices[i]-prices[j] {
				maxProfit = maxPreprofit + prices[i] - prices[j]
			}
		}
		profit[i] = maxProfit
		if result < maxProfit {
			result = maxProfit
		}
	}

	return result
}
