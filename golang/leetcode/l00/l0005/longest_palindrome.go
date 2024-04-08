package l0005

func longestPalindrome(s string) string {
	sb := []byte(s)
	n := len(sb)
	check := make([][]bool, n)
	resultStart := 0
	resultLen := 0
	for i := range check {
		check[i] = make([]bool, n-i+1)
		check[i][0] = true
		check[i][1] = true
		resultStart = i
		resultLen = 1
	}

	for len := 2; len <= n; len++ {
		for i := 0; i <= n-len; i++ {
			if sb[i] == sb[i+len-1] && check[i+1][len-2] {
				check[i][len] = true
				resultStart = i
				resultLen = len
			}
		}
	}

	// for i, v := range check {
	// 	fmt.Println(i, v)
	// }

	return string(sb[resultStart : resultStart+resultLen])
}
