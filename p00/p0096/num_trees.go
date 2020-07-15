package p0096

func numTrees(n int) int {
	nums := make([]int, n+1)

	nums[0] = 1
	nums[1] = 1
	for i := 2; i <= n; i++ {
		count := 0
		for j := 0; j < i; j++ {
			count += nums[j] * nums[i-j-1]
		}

		nums[i] = count
	}

	return nums[n]
}
