package l5450

import (
	"fmt"
	"sort"
)

//cspell:ignore subseq

const mod = 1000_000_007

var power []int

func numSubseq(nums []int, target int) int {
	power = make([]int, len(nums)+1)
	power[0] = 1
	sort.Slice(nums, func(i, j int) bool { return nums[i] < nums[j] })
	fmt.Println(nums, target)
	result := 0
	length := len(nums)
	start := sort.Search(length+1, func(n int) bool {
		return n >= length || nums[n] >= target/2+1
	})

	result = add(result, 0, start)

	fmt.Println("start", start, 1, result)
	for i := start; i < length; i++ {
		other := target - nums[i]
		j := sort.Search(i, func(n int) bool {
			return n > i || nums[n] > other
		}) - 1
		result = add(result, i-j-1, j+1)
		fmt.Println("find", i, j, "para", i-j-1, j+1, result)
	}
	return result
}

func add(a, p1, p2 int) int {
	if p1 < 0 {
		p1 = 0
	}
	r := (getPower(p1) * (getPower(p2) - 1)) % mod
	return (a + r) % mod
}

func getPower(n int) int {
	curr := n
	for ; power[curr] == 0; curr-- {
	}

	curr++
	for ; curr <= n; curr++ {
		power[curr] = power[curr-1] * 2 % mod
	}
	return power[n]
}
