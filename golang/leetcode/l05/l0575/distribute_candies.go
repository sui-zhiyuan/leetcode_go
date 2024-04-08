package l0575

func distributeCandies(candies []int) int {
	candyMap := map[int]struct{}{}
	for _, v := range candies {
		if _, ok := candyMap[v]; !ok {
			candyMap[v] = struct{}{}
		}
	}

	kind := len(candyMap)
	count := len(candies)
	if kind <= count/2 {
		return kind
	}
	return count / 2
}
