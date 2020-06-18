package p0002

// ListNode ...
type ListNode struct {
	Val  int
	Next *ListNode
}

// NewListNode ...
func NewListNode(v int) *ListNode {
	result := new(ListNode)
	for k := result; v > 0; v /= 10 {
		k.Next = new(ListNode)
		k = k.Next
		k.Val = v % 10
	}
	return result.Next
}
