package l0002

import "github.com/sui-zhiyuan/leetcode_go/golang/common"

// ListNode ...
type ListNode = common.ListNode

//lint:file-ignore U1000 leetcode
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	upper := 0
	result := new(ListNode)
	k := result
	for i, j := l1, l2; i != nil || j != nil; {
		k.Next = new(ListNode)
		k = k.Next
		v := upper
		if i != nil {
			v += i.Val
			i = i.Next
		}
		if j != nil {
			v += j.Val
			j = j.Next
		}
		k.Val = v % 10
		upper = v / 10
	}
	if upper > 0 {
		k.Next = new(ListNode)
		k = k.Next
		k.Val = upper
	}
	return result.Next
}
