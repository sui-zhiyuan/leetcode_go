package p1140

func stoneGameII(piles []int) int {
	length := len(piles)
	statusArea := make([][]status, length)
	for i := length - 1; i >= 0; i-- {
		statusArea[i] = make([]status, length+1)
		for m := 1; m <= length; m++ {
			maxStatus := status{}
			for k := 1; k <= m*2; k++ {
				curr := status{}
				if i+k < length {
					curr = statusArea[i+k][max(k, m)]
				}

				if curr.loss+sum(piles, i, i+k) > maxStatus.got {
					maxStatus = status{
						got:  curr.loss + sum(piles, i, i+k),
						loss: curr.got,
					}
				}
			}
			statusArea[i][m] = maxStatus
		}
	}

	// for i, v := range statusArea {
	// 	fmt.Println("line", i, v)
	// }
	return statusArea[0][1].got
}

type status struct {
	got  int
	loss int
}

func sum(piles []int, start, end int) int {
	if end >= len(piles) {
		end = len(piles)
	}

	result := 0
	for i := start; i < end; i++ {
		result += piles[i]
	}

	return result
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
