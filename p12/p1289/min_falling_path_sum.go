package p1289

func minFallingPathSum(arr [][]int) int {
	curr, prev := status{}, status{}

	for _, line := range arr {
		prev = curr
		curr = status{
			mini:       -1,
			secondMini: -1,
			miniCol:    -1,
		}
		for j, v := range line {
			result := v
			if j == prev.miniCol {
				result += prev.secondMini
			} else {
				result += prev.mini
			}
			if curr.mini == -1 || curr.mini > result {
				curr.secondMini = curr.mini
				curr.mini = result
				curr.miniCol = j
				continue
			}
			if curr.secondMini == -1 || curr.secondMini > result {
				curr.secondMini = result
			}
		}
		//fmt.Printf("%+v\n", curr)
	}

	return curr.mini
}

type status struct {
	mini       int
	secondMini int
	miniCol    int
}
