package l1032

// StreamChecker ...
type StreamChecker struct {
	nodes []node

	curr int
}

// Constructor ...
func Constructor(words []string) StreamChecker {
	sc := StreamChecker{
		nodes: make([]node, 0, len(words)*13),
		curr:  1,
	}

	sc.nodes = append(sc.nodes, node{}, node{})
	for _, w := range words {
		curr := 1
		for i := 0; i < len(w); i++ {
			c := w[i] - 'a'
			next := sc.nodes[curr].next[c]
			if next == 0 {
				sc.nodes = append(sc.nodes, node{})
				next = len(sc.nodes) - 1
				sc.nodes[curr].next[c] = next
			}
			curr = next
		}
		sc.nodes[curr].final = true
	}

	dummy := 0
	head := 1
	for i := 0; i < 26; i++ {
		sc.nodes[dummy].next[i] = head
	}
	sc.nodes[dummy].fail = dummy
	sc.nodes[head].fail = dummy

	q := &intQueue{}
	q.enqueue(head)

	for curr, ok := q.dequeue(); ok; curr, ok = q.dequeue() {
		for i := 0; i < 26; i++ {
			cfail := sc.nodes[curr].fail
			if sc.nodes[curr].next[i] == 0 {
				sc.nodes[curr].next[i] = sc.nodes[cfail].next[i]
				continue
			}
			next := sc.nodes[curr].next[i]
			sc.nodes[next].fail = sc.nodes[cfail].next[i]
			q.enqueue(next)
		}
	}

	sc.print()

	return sc
}

// Query ...
func (sc *StreamChecker) Query(letter byte) bool {
	sc.curr = sc.nodes[sc.curr].next[letter-'a']
	fail := sc.curr
	for fail > 0 {
		if sc.nodes[fail].final {
			return true
		}
		fail = sc.nodes[fail].fail
	}
	return false
}

func (sc *StreamChecker) print() {
	// for i := range sc.nodes {
	// 	fmt.Printf("%02d : ", i)
	// 	fmt.Printf("fail->%02d ", sc.nodes[i].fail)
	// 	for j := 0; j < 26; j++ {
	// 		if sc.nodes[i].next[j] != 1 {
	// 			fmt.Printf("%s->%02d ", string(byte('a'+j)), sc.nodes[i].next[j])
	// 		}
	// 	}
	// 	fmt.Printf("%t\n", sc.nodes[i].final)
	// }
}

type node struct {
	next  [26]int
	fail  int
	final bool
}

type intQueue struct {
	head *intQueueNode
	tail *intQueueNode
}

type intQueueNode struct {
	val  int
	next *intQueueNode
}

func (q *intQueue) enqueue(v int) {
	if q.tail == nil {
		q.tail = &intQueueNode{val: v}
		q.head = q.tail
		return
	}
	q.head.next = &intQueueNode{val: v}
	q.head = q.head.next
}

func (q *intQueue) dequeue() (v int, ok bool) {
	if q.tail == nil {
		return 0, false
	}

	v = q.tail.val
	q.tail = q.tail.next
	return v, true
}
