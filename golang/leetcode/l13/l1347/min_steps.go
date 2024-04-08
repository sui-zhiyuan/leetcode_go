package l1347

func minSteps(s string, t string) int {
	frequency := make([]int, 26)

	for _, c := range s {
		frequency[c-'a']++
	}
	for _, c := range t {
		frequency[c-'a']--
	}

	result := 0
	for _, v := range frequency {
		if v > 0 {
			result += v
		}
	}

	return result
}
