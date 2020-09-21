package p0538

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// TreeNode ...
type TreeNode = define.TreeNode

func convertBST(root *TreeNode) *TreeNode {
	curr := root
	s := &stack{}
	sum := 0
	for {
		// fmt.Println("start root", curr)
		for curr != nil {
			s.push(curr)
			curr = curr.Right
		}
		var ok bool
		curr, ok = s.pop()
		if !ok {
			break
		}
		curr.Val += sum
		sum = curr.Val
		curr = curr.Left
		// fmt.Println("left value", curr)
	}
	return root
}

type stack struct {
	top   int
	array []*TreeNode
}

func (s *stack) push(v *TreeNode) {
	// defer func() {
	// 	fmt.Println("push", v)
	// 	fmt.Println("top", s.top, "array", s.array)
	// }()
	length := len(s.array)
	if length == s.top {
		newArray := make([]*TreeNode, length*2+1)
		copy(newArray, s.array)
		s.array = newArray
	}
	s.array[s.top] = v
	s.top++
}

func (s *stack) pop() (v *TreeNode, ok bool) {
	// defer func() {
	// 	fmt.Println("pop")
	// 	fmt.Println("top", s.top, "array", s.array, "v", v, "ok", ok)
	// }()
	if s.top == 0 {
		return nil, false
	}
	s.top--
	return s.array[s.top], true
}

func (s *stack) peak() (v *TreeNode, ok bool) {
	// defer func() {
	// 	fmt.Println("peak")
	// 	fmt.Println("top", s.top, "array", s.array, "v", v, "ok", ok)
	// }()
	if s.top == 0 {
		return nil, false
	}
	return s.array[s.top-1], true
}
