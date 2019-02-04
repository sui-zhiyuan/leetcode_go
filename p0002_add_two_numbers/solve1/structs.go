package solve1

type ListNode struct {
	Val  int
	Next *ListNode
}

func newListNode(v int) *ListNode {
	result := new(ListNode)
	for k := result; v > 0; v /= 10 {
		k.Next = new(ListNode)
		k = k.Next
		k.Val = v % 10
	}
	return result.Next
}
