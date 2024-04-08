package l5449

import (
	"sort"
)

func canArrange(arr []int, k int) bool {
	for i := range arr {
		arr[i] = arr[i] % k
		if arr[i] < 0 {
			arr[i] += k
		}
	}

	sort.Slice(arr, func(i, j int) bool {
		return arr[i] < arr[j]
	})

	length := len(arr)
	for i, j := 0, length-1; i < j; {
		if arr[i] == 0 && arr[i+1] == 0 {
			i += 2
			continue
		}
		if arr[i] == 0 && arr[i+1] != 0 {
			return false
		}
		if arr[i]+arr[j] != k {
			return false
		}
		i++
		j--
	}

	return true
}
