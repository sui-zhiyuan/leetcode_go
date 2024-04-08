package l1213

const maxValue = 2001

func arraysIntersection(arr1 []int, arr2 []int, arr3 []int) []int {
	arr := make([][]int, 3)
	arr[0] = arr1
	arr[1] = arr2
	arr[2] = arr3

	curr := make([]int, 3)
	result := make([]int, 0, len(arr1))

	for {
		min := maxValue
		count := 0
		loc := 0
		for i, v := range arr {
			if curr[i] < len(v) && v[curr[i]] < min {
				count = 1
				loc = i
				min = v[curr[i]]
				continue
			}
			if curr[i] < len(v) && v[curr[i]] == min {
				count++
			}
		}

		if min == maxValue {
			break
		}

		if count == 3 {
			result = append(result, min)
			for i := range curr {
				curr[i]++
			}
			continue
		}

		curr[loc]++
	}

	return result
}
