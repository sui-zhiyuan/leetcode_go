package p0139

func wordBreak(s string, wordDict []string) bool {
	t := newTrie(wordDict)
	t.print()

	wordLocations := make([][]int, len(s))
	curr := 1
	for i := 0; i < len(s); i++ {
		next := t.getNode(curr, s[i])
		if next == 0 {
			next = 1
		}
		fail := next
		for fail > 0 {
			for _, w := range t.nodes[fail].final {
				wordLocations[i-len(wordDict[w])+1] = append(wordLocations[i-len(wordDict[w])+1], i+1)
			}
			fail = t.nodes[fail].fail
		}
		curr = next
	}

	//fmt.Println(wordLocations)

	q := queueInt{}
	q.enqueue(0)
	lens := len(s)
	arrive := make([]bool, lens)
	arrive[0] = true

	for v, ok := q.dequeue(); ok; v, ok = q.dequeue() {
		for _, next := range wordLocations[v] {
			if next == lens {
				return true
			}
			if !arrive[next] {
				arrive[next] = true
				q.enqueue(next)
			}
		}
	}

	return false
}

type trie struct {
	nodes     []node
	charCount int
	charMap   map[byte]int
}

type node struct {
	next  []int
	fail  int
	final []int
}

func newTrie(words []string) *trie {
	t := &trie{
		charMap: make(map[byte]int),
	}

	dummy := t.newNode()
	start := t.newNode()

	for i, w := range words {
		curr := start
		for j := 0; j < len(w); j++ {
			var next int
			if next = t.getNode(curr, w[j]); next == 0 {
				next = t.newNode()
				t.setNode(curr, w[j], next)
			}
			curr = next
		}
		t.nodes[curr].final = append(t.nodes[curr].final, i)
	}

	for i := range t.charMap {
		t.setNode(dummy, i, start)
	}
	t.nodes[dummy].fail = dummy
	t.nodes[start].fail = dummy

	q := &queueInt{}
	q.enqueue(start)

	for curr, ok := q.dequeue(); ok; curr, ok = q.dequeue() {
		for i := range t.charMap {
			next := t.getNode(curr, i)
			currFail := t.nodes[curr].fail
			if next == 0 {
				t.setNode(curr, i, t.getNode(currFail, i))
				continue
			}
			t.nodes[next].fail = t.getNode(currFail, i)
			q.enqueue(next)
		}
	}

	return t
}

func (t *trie) newNode() int {
	length := len(t.nodes)
	t.nodes = append(t.nodes, node{})
	return length
}

func (t *trie) getNode(curr int, c byte) int {
	i, ok := t.charMap[c]
	if !ok {
		return 0
	}
	t.extend(curr, i)
	return t.nodes[curr].next[i]
}

func (t *trie) setNode(curr int, c byte, next int) {
	i, ok := t.charMap[c]
	if !ok {
		i = t.charCount
		t.charMap[c] = i
		t.charCount++
	}
	t.extend(curr, i)
	t.nodes[curr].next[i] = next
}

func (t *trie) extend(curr int, i int) {
	if i >= len(t.nodes[curr].next) {
		temp := t.nodes[curr].next
		t.nodes[curr].next = make([]int, t.charCount)
		copy(t.nodes[curr].next, temp)
	}
}

func (t *trie) print() {
	// chars := []byte{}
	// for i := range t.charMap {
	// 	chars = append(chars, i)
	// }
	// sort.Slice(chars, func(i, j int) bool { return chars[i] < chars[j] })
	// for i, v := range t.nodes {
	// 	fmt.Printf("%3d : fail->%d ", i, v.fail)
	// 	for _, j := range chars {
	// 		next := t.getNode(i, j)
	// 		if next != 1 {
	// 			fmt.Printf("%s->%d ", string(byte(j)), next)
	// 		}
	// 	}
	// 	fmt.Printf("final%+v\n", v.final)
	// }
}

type queueInt struct {
	head *queueNode
	tail *queueNode
}

type queueNode struct {
	value int
	next  *queueNode
}

func (q *queueInt) enqueue(v int) {
	next := &queueNode{
		value: v,
	}
	if q.tail == nil {
		q.head = next
		q.tail = next
		return
	}

	q.head.next = next
	q.head = next
}

func (q *queueInt) dequeue() (v int, ok bool) {
	if q.tail == nil {
		return 0, false
	}

	v = q.tail.value
	q.tail = q.tail.next
	return v, true
}
