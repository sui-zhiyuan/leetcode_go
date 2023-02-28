package p2q1

const times = 2

func maxProfit(values []int) int {
	length := len(values)
	maxProfit := make([][]int, len(values))
	for i := range maxProfit {
		maxProfit[i] = make([]int, times+1)
	}
	minValue := values[0]
	minLoc := 0

	for i := 1; i < length; i++ {
		for j := 1; j <= times; j++ {
			maxProfit[i][j] = max(maxProfit[i][j-1], maxProfit[i-1][j], maxProfit[minLoc][j-1]+values[i]-minValue)

		}
		if values[i] < minValue {
			minValue = values[i]
			minLoc = i
		}
	}

	return maxProfit[length][times]
}

func max(t ...int) int {
	result := t[0]
	for _, v := range t[1:] {
		if result < v {
			result = v
		}
	}
	return result
}
