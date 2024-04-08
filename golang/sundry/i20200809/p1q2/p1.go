package p1q2

func Find(a, b []int) int {
	check := func(i int) bool { return a[i] == b[i] }
	return bSearch(0, len(a), check)
}

func bSearch(start int, end int, check func(i int) bool) int {
	if start == end {
		return start
	}
	if start+1 == end {
		if check(start) {
			return end
		}
		return start
	}
	z := (start + end) / 2
	if check(z) {
		return bSearch(z+1, end, check)
	}

	return bSearch(start, z+1, check)

}
