package li1610

const start = 1900

func maxAliveYear(birth []int, death []int) int {
	years := make([]int, 102)

	for _, v := range birth {
		years[v-start]++
	}
	for _, v := range death {
		years[v-start+1]--
	}

	max := 0
	maxV := 0
	curr := 0
	for i, v := range years {
		// fmt.Println(i+start, v, curr, maxV)
		curr += v
		if curr > maxV {
			maxV = curr
			max = i
		}
	}

	return start + max
}
