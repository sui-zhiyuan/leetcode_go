package p1347

func minSteps(s string, t string) int {
	freqency := make([]int, 26)

	for _, c := range s {
		freqency[c-'a']++
	}
	for _, c := range t {
		freqency[c-'a']--
	}

	result := 0
	for _, v := range freqency {
		if v > 0 {
			result += v
		}
	}

	return result
}
