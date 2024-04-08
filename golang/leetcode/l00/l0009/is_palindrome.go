package l0009

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	digs := make([]int, 0, 21)

	for x > 0 {
		digs = append(digs, x%10)
		x = x / 10
	}

	for i, j := 0, len(digs)-1; i < j; {
		if digs[i] != digs[j] {
			return false
		}
		i++
		j--
	}
	return true
}
