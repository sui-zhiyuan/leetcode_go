package l0986

func intervalIntersection(A [][]int, B [][]int) [][]int {
	result := [][]int{}

	currA, currB := 0, 0
	lenA, lenB := len(A), len(B)

	for {
		if currA >= lenA || currB >= lenB {
			return result
		}

		switch {
		case A[currA][0] < B[currB][0] && A[currA][1] < B[currB][0]:
			currA++
		case B[currB][0] < A[currA][0] && B[currB][1] < A[currA][0]:
			currB++
		case A[currA][0] <= B[currB][0] && A[currA][1] <= B[currB][1]:
			result = append(result, []int{B[currB][0], A[currA][1]})
			currA++
		case A[currA][0] <= B[currB][0] && A[currA][1] > B[currB][1]:
			result = append(result, []int{B[currB][0], B[currB][1]})
			currB++
		case A[currA][0] > B[currB][0] && A[currA][1] <= B[currB][1]:
			result = append(result, []int{A[currA][0], A[currA][1]})
			currA++
		case A[currA][0] > B[currB][0] && A[currA][1] > B[currB][1]:
			result = append(result, []int{A[currA][0], B[currB][1]})
			currB++
		}
	}
}
