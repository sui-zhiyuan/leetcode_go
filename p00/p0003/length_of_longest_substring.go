package p0003

func lengthOfLongestSubstring(s string) int {
	usedFlag := [256]bool{}
	ans := 0
	head, tail := 0, 0
	length := len(s)

	for {
		switch {
		case head == length:
			return ans

		case !usedFlag[s[head]]:
			usedFlag[s[head]] = true
			head++
			if head-tail > ans {
				ans = head - tail
			}
		default:
			usedFlag[s[tail]] = false
			tail++
		}
	}
	// unreachable code
}
