package li0803

func findMagicIndex(nums []int) int {
	for i, v := range nums {
		if i == v {
			return i
		}
	}
	return -1
}
