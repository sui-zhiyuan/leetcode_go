package p5452

import (
	"sort"
)

func canMakeArithmeticProgression(arr []int) bool {
	sort.Slice(arr, func(i, j int) bool {
		return arr[i] < arr[j]
	})

	for i := 0; i < len(arr)-2; i++ {
		if arr[i+1]*2 != arr[i+2]+arr[i] {
			return false
		}
	}

	return true
}
