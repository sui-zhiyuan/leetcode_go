package p5435

func minNumberOfSemesters(n int, dependencies [][]int, k int) int {
	matrix := make([][]bool, n)
	for i := range matrix {
		matrix[i] = make([]bool, n)
	}

	for _, d := range dependencies {
		matrix[d[0]-1][d[1]-1] = true
	}
	postLen := make([]int, n)

	for i := 0; i < n; i++ {
		_ = getPost(matrix, postLen, n, i)
	}

	//fmt.Println("postLen", postLen)

	group := make([]int, n)
	curr := 1
	set := 0
	total := 0
	for total < n {
		got := false
		for i := n; i >= 1; i-- {
			for c := 0; c < n; c++ {
				if postLen[c] == i && group[c] == 0 {
					if check(matrix, group, n, c, curr) {
						group[c] = curr
						set++
						total++
						got = true
					}
				}
				if got {
					break
				}
			}
			if got {
				break
			}
		}

		//fmt.Println("step", group, "curr", curr, "set", set, "total", total, "got", got)

		if got && set == k {
			curr++
			set = 0
			continue
		}

		if !got {
			curr++
			set = 0
			continue
		}

	}

	//fmt.Println("group", group)

	if set == 0 {
		curr--
	}
	return curr
}

func getPost(matrix [][]bool, postLen []int, n int, curr int) int {
	if postLen[curr] > 0 {
		return postLen[curr]
	}

	max := 0
	for i := 0; i < n; i++ {
		if matrix[curr][i] {
			c := getPost(matrix, postLen, n, i)
			if c > max {
				max = c
			}
		}
	}

	postLen[curr] = max + 1
	return postLen[curr]
}

func check(matrix [][]bool, group []int, n int, x, curr int) bool {
	//fmt.Println("check-in", "x", x, "curr", curr)
	for i := 0; i < n; i++ {
		if matrix[i][x] && (group[i] == 0 || group[i] >= curr) {
			//fmt.Println("check-in", "x", x, "curr", curr, "i", i, "false")
			return false
		}
	}
	//fmt.Println("check-in", "x", x, "curr", curr, "true")
	return true
}
