package l0315

import (
	"sort"
)

// cspell:ignore znum

//lint:ignore U1000 leetcode requirement
func countSmaller(nums []int) []int {
	length := len(nums)
	znum := make([]int, length)

	t := newBinaryIndexedTree(znum)
	nodes := make([]node, length)
	for i, v := range nums {
		nodes[i] = node{v, i}
	}

	sort.Slice(nodes, func(i, j int) bool {
		if nodes[i].value == nodes[j].value {
			return nodes[i].index < nodes[j].index
		}
		return nodes[i].value < nodes[j].value
	})

	result := make([]int, length)

	for _, n := range nodes {
		result[n.index] = t.calculateSum(n.index, length)
		t.update(n.index, 1)
	}

	return result
}

type node struct {
	value int
	index int
}

type binaryIndexedTree struct {
	nums []int
	sums []int
}

func newBinaryIndexedTree(nums []int) *binaryIndexedTree {
	t := &binaryIndexedTree{}
	t.nums = make([]int, len(nums))
	t.sums = make([]int, len(nums))

	copy(t.nums, nums)
	for i, v := range t.nums {
		t.updateValue(i, v)
	}
	return t
}

func (t *binaryIndexedTree) update(i, v int) {
	t.updateValue(i, v-t.nums[i])
	t.nums[i] = v
}

func (t *binaryIndexedTree) updateValue(i, v int) {
	len := len(t.sums)
	i = i + 1
	for i <= len {
		t.sums[i-1] += v
		i += getLower(i)
	}
}

func (t *binaryIndexedTree) calculateSum(i, j int) int {
	result := 0

	for i != j {
		if j > i {
			result += t.sums[j-1]
			j -= getLower(j)
			continue
		}
		result -= t.sums[i-1]
		i -= getLower(i)
	}
	return result
}

func getLower(i int) int {
	return i & -i
}
