package l0718

func findLength(A []int, B []int) int {
	prev := make([]int, len(A))
	curr := make([]int, len(A))

	ans := 0
	for _, vB := range B {
		for i := range prev {
			prev[i] = 0
		}
		curr, prev = prev, curr

		for i, vA := range A {
			p1 := 0
			if i > 0 {
				p1 = prev[i-1]
			}
			if vA == vB {
				curr[i] = p1 + 1
				if curr[i] > ans {
					ans = curr[i]
				}
			}

			//fmt.Println("step", p1, p2, p3, vA, vB)
		}

		//fmt.Println("roud", vB, curr)
	}

	return ans
}
