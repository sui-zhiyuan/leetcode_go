package p1299

func replaceElements(arr []int) []int {
	result := make([]int, len(arr))
	curr := -1
	for i := len(arr) - 1; i >= 0; i-- {
		result[i] = curr
		if curr < arr[i] {
			curr = arr[i]
		}
	}
	return result
}
