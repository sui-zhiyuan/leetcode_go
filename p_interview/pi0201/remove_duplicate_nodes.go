package pi0201

import (
	"github.com/sui-zhiyuan/leetcode_go/define"
)

// ListNode ...
type ListNode = define.ListNode

func removeDuplicateNodes(head *ListNode) *ListNode {
	sum := 0
	var prev, curr *ListNode
	for prev, curr = nil, head; curr != nil; prev, curr = curr, curr.Next {
		if curr.Val|sum == sum {
			for check := head; check != curr; check = check.Next {
				if check.Val == curr.Val {
					prev.Next = curr.Next
					curr = prev
					break
				}
			}
			continue
		}
		sum = curr.Val | sum
	}
	return head
}
