package p1q1

import (
	"fmt"

	"github.com/sui-zhiyuan/leetcode_go/golang/common"
)

// ListNode ...
type ListNode = common.ListNode

// Revert ...
func Revert(head *ListNode) *ListNode {
	fmt.Println("123123")
	newHead, _ := revert(head)

	for curr := newHead; curr != nil; curr = curr.Next {
		fmt.Printf("%+v\n", curr)
	}
	return newHead
}

func revert(head *ListNode) (newHead *ListNode, newTail *ListNode) {
	if head.Next == nil {
		newHead = head
		newTail = head
		return newHead, newTail
	}
	newHead, newTail = revert(head.Next)
	fmt.Println("before", "newHead", newHead.Val, "newTail", newTail.Val)
	newTail.Next = head
	head.Next = nil
	newTail = newTail.Next
	fmt.Println("after", "newHead", newHead.Val, "newTail", newTail.Val)
	return newHead, newTail
}
