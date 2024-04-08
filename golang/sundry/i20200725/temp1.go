package i20200725

import (
	"fmt"
)

func calculate(a [][]int) int {
	result := make([][]int, len(a))

	result[0] = make([]int, 1)
	result[0][0] = a[0][0]
	fmt.Println(result[0])
	for i, v := range a[1:] {
		result[i+1] = make([]int, len(v))
		for j := range result[i+1] {
			pres := make([]int, 0, 2)
			if j >= 1 {
				pres = append(pres, result[i][j-1])
			}
			if j < len(v)-1 {
				pres = append(pres, result[i][j])
			}
			result[i+1][j] = max(pres...) + a[i+1][j]
		}
		fmt.Println(result[i+1])
	}

	return max(result[len(result)-1]...)
}

func max(v ...int) int {
	max := v[0]
	for _, c := range v[1:] {
		if c > max {
			max = c
		}
	}
	return max
}
