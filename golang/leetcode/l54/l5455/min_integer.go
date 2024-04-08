package l5455

func minInteger(num string, k int) string {
	nums := []byte(num)
	n := len(nums)
	loc := [10][]int{}
	for i, v := range nums {
		loc[v-'0'] = append(loc[v-'0'], i)
	}

	// fmt.Println("nums", string(nums), "k", k)
	// for i := 0; i < 10; i++ {
	// 	fmt.Println(i, loc[i])
	// }

	values := make([]int, n)
	for i := range values {
		values[i] = 1
	}
	tree := newBinaryIndexedTree(values)

	result := make([]byte, 0, n)
	curr := [10]int{}
	for k > 0 {
		found := false
		for i, j := range curr {
			if j >= len(loc[i]) {
				continue
			}

			if step := tree.sum(loc[i][j]); step <= k {
				result = append(result, '0'+byte(i))
				k -= step
				tree.update(loc[i][j], 0)
				curr[i]++
				found = true

				// fmt.Println("found", i, j, loc[i][j], "k", k, "result", string(result), tree.nums)

				break
			}
		}

		if !found {
			break
		}
	}

	for i := 0; i < n; i++ {
		if tree.nums[i] != 0 {
			result = append(result, nums[i])
		}
	}

	return string(result)
}

type binaryIndexedTree struct {
	nums  []int
	sums  []int
	count int
}

func newBinaryIndexedTree(nums []int) *binaryIndexedTree {
	count := len(nums)
	tree := &binaryIndexedTree{
		nums:  nums,
		count: count,
		sums:  make([]int, count),
	}

	for i, v := range nums {
		tree.updateValue(i+1, v)
	}

	return tree
}

func (tree *binaryIndexedTree) update(i int, v int) {
	tree.updateValue(i+1, v-tree.nums[i])
	tree.nums[i] = v
}

func (tree *binaryIndexedTree) updateValue(i int, sub int) {
	for i <= tree.count {
		tree.sums[i-1] += sub
		i += lowerBit(i)
	}
}

func (tree *binaryIndexedTree) sum(i int) int {
	sum := 0
	for i > 0 {
		sum += tree.sums[i-1]
		i -= lowerBit(i)
	}

	return sum
}

func (tree *binaryIndexedTree) sumBetween(i, j int) int {
	sum := 0
	for i != j {
		if j > i {
			sum += tree.sums[j-1]
			j -= lowerBit(j)
			continue
		}
		sum -= tree.sums[i-1]
		i -= lowerBit(i)
	}
	return sum
}

func lowerBit(i int) int {
	return i & (-i)
}
