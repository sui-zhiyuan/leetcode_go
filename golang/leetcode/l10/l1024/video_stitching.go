package l1024

func videoStitching(clips [][]int, T int) int {
	endEdge := make([][]bool, T+1)
	for i := range endEdge {
		endEdge[i] = make([]bool, i)
	}
	for _, v := range clips {
		if v[1] > T {
			v[1] = T
		}
		for k := v[0] + 1; k <= v[1]; k++ {
			endEdge[k][v[0]] = true
		}
	}

	// for i, v := range endEdge {
	// 	fmt.Println(i, v)
	// }

	miniClip := make([]int, T+1)
	miniClip[0] = 0
	for i := 1; i <= T; i++ {
		min := -1
		for j := 0; j < i; j++ {
			if !endEdge[i][j] || miniClip[j] == -1 {
				continue
			}
			if min == -1 || miniClip[j]+1 < min {
				min = miniClip[j] + 1
			}
		}
		miniClip[i] = min
	}

	return miniClip[T]
}
