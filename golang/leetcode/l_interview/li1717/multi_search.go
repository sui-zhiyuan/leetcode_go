package li1717

func multiSearch(big string, smalls []string) [][]int {
	start := generateTree(smalls)
	generateJump(start)
	return findMatch(big, smalls, start)
}

func generateTree(smalls []string) *node {
	start := &node{}
	for i, s := range smalls {
		curr := start
		for j := 0; j < len(s); j++ {
			var next *node
			if next = curr.getNext(s[j]); next == nil {
				next = &node{}
				curr.setNext(s[j], next)
			}
			if j == len(s)-1 {
				next.final = append(next.final, i)
			}
			curr = next
		}
	}
	return start
}

func generateJump(start *node) {
	dummy := &node{}
	for c := byte('a'); c <= 'z'; c++ {
		dummy.setNext(c, start)
	}
	start.jump = dummy
	queue := &queueSet{}
	queue.enqueue(start)

	for {
		curr, ok := queue.dequeue()
		if !ok {
			break
		}

		for c := byte('a'); c <= 'z'; c++ {
			next := curr.getNext(c)
			if next == nil {
				curr.setNext(c, curr.jump.getNext(c))
				continue
			}
			next.jump = curr.jump.getNext(c)
			queue.enqueue(next)
		}
	}
}

func findMatch(big string, smalls []string, start *node) [][]int {
	result := make([][]int, len(smalls))
	for i := range result {
		result[i] = []int{}
	}
	curr := start
	for i := 0; i < len(big); i++ {
		curr = curr.getNext(big[i])
		for jump := curr; jump != nil; jump = jump.jump {
			for _, v := range jump.final {
				result[v] = append(result[v], i-len(smalls[v])+1)
			}
		}
	}
	return result
}

type node struct {
	next  ['z' - 'a' + 1]*node
	final []int
	jump  *node
}

func (n *node) getNext(c byte) *node {
	return n.next[c-'a']
}

func (n *node) setNext(c byte, next *node) {
	n.next[c-'a'] = next
}

type queueSet struct {
	head *queueNode
	tail *queueNode
}

type queueNode struct {
	value *node
	next  *queueNode
}

func (q *queueSet) enqueue(v *node) {
	if q.head == nil {
		q.head = &queueNode{
			value: v,
		}
		q.tail = q.head
		return
	}

	next := &queueNode{
		value: v,
	}
	q.head.next = next
	q.head = next
}

func (q *queueSet) dequeue() (v *node, ok bool) {
	if q.tail == nil {
		return nil, false
	}

	v = q.tail.value
	if q.tail.next == nil {
		q.head = nil
	}

	q.tail = q.tail.next
	return v, true
}
