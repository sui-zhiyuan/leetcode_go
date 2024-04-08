package l1163

func lastSubstring(s string) string {
	sb := []byte(s)

	// for i := range sb {
	// 	fmt.Print(i % 10)
	// }
	// fmt.Println()
	// fmt.Println(s)

	max := 0
	next := -1
	n := len(sb)
	for i := 0; i < n; i++ {
		if sb[i] > sb[max] {
			// fmt.Println("update max", max, i, string(byte(sb[i])), string(byte(sb[max])))
			max = i
			next = -1
			continue
		}

		if sb[i] == sb[max] && max != i && next == -1 {
			// fmt.Println("found next", i)
			next = i
		}
		if next >= 0 {
			if sb[max+i-next] > sb[i] {
				// fmt.Println("next to zero", next, i)
				next = -1
				continue
			}
			if sb[max+i-next] < sb[i] {
				// fmt.Println("update max to next", max, next, i)
				max = next
				next = -1
				i = max
			}
		}
	}

	return string(sb[max:])
}
