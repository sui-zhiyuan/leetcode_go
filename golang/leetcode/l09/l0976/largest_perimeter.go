package l0976

import (
	"sort"
)

func largestPerimeter(A []int) int {
	sort.Slice(A, func(i, j int) bool { return A[i] > A[j] })
	n := len(A)
	for i := 0; i < n-2; i++ {
		if A[i+1]+A[i+2] > A[i] {
			return A[i] + A[i+1] + A[i+2]
		}
	}
	return 0
}
