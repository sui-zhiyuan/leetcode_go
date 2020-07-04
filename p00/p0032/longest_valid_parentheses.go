package p0032

func longestValidParentheses(s string) (result int) {
	barr := []byte(s)

	result = 0
	leftLoc := []int{}
	start := -1
	for i, b := range barr {
		//fmt.Println("step", i, string(b), leftLoc, start)
		if b == '(' && len(leftLoc) == 0 && start == -1 {
			start = i
		}
		if b == '(' {
			leftLoc = append(leftLoc, i)
			continue
		}

		if b == ')' && len(leftLoc) == 0 {
			if start >= 0 && result < i-start {
				result = i - start
			}
			start = -1
			continue
		}

		leftLoc = leftLoc[:len(leftLoc)-1]
	}
	//fmt.Println("final", leftLoc, start)

	if start >= 0 {
		leftLoc = append(leftLoc, len(barr))
	}
	for i, v := range leftLoc {
		left := start - 1
		if i >= 1 {
			left = leftLoc[i-1]
		}

		if v-left-1 > result {
			result = v - left - 1
		}
	}

	return
}
