package l0982

const maxInt16 = 0b1_0000_0000_0000_0000

func countTriplets(A []int) int {
	max := 0
	for _, v := range A {
		max = max | v
	}

	prev := make([]int, max+1)
	curr := make([]int, max+1)
	zero := make([]int, max+1)
	curr[max] = 1
	for times := 0; times < 3; times++ {
		copy(prev, curr)
		copy(curr, zero)
		for i := 0; i <= max; i++ {
			for _, v := range A {
				curr[i&v] += prev[i]
			}
		}
	}

	return curr[0]
}
