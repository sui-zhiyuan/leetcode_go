package l1506

func findRoot(tree []*Node) *Node {
	find := []int{}

	for _, node := range tree {
		for node.Val >= len(find) {
			find = append(find, 0)
		}
		find[node.Val] |= 1

		for _, child := range node.Children {
			for child.Val >= len(find) {
				find = append(find, 0)
			}

			find[child.Val] |= 2
		}
	}

	fv := -1
	for v := range find {
		if find[v] == 1 {
			fv = v
			break
		}
	}

	for _, node := range tree {
		if node.Val == fv {
			return node
		}
	}

	panic("unreachable")
}

type Node struct {
	Val      int
	Children []*Node
}
