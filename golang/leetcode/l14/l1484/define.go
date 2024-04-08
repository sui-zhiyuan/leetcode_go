package l1484

import (
	"math"
)

// Node ...
type Node struct {
	Val    int
	Left   *Node
	Right  *Node
	Random *Node
}

// NodeCopy ...
type NodeCopy = Node

// Nil ...
const Nil = math.MinInt32

func newTree(value []int, random []int) *Node {
	nodes := make([]*Node, len(value))
	for i, v := range value {
		if v != Nil {
			nodes[i] = &Node{Val: v}
		}
	}

	for i, n := range nodes {
		if n == nil {
			continue
		}
		if i*2+1 < len(nodes) {
			n.Left = nodes[i*2+1]
		}
		if i*2+2 < len(nodes) {
			n.Right = nodes[i*2+2]
		}
	}

	for i, n := range random {
		if n != Nil {
			nodes[i].Random = nodes[n]
		}
	}

	return nodes[0]
}
