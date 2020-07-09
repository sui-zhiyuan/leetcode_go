package pi1713

func respace(dictionary []string, sentence string) int {
	sentenceb := []byte(sentence)
	length := len(sentence)
	jump := make([][]int, length)
	t := newTire(dictionary)

	curr := 1
	for i, c := range sentenceb {
		curr = t.nodes[curr].next[c-'a']

		for fail := curr; fail > 0; fail = t.nodes[fail].fail {
			for _, final := range t.nodes[fail].final {
				jump[i] = append(jump[i], i-final)
			}
		}
	}

	// for i, v := range jump {
	// 	fmt.Println(i, v)
	// }

	result := make([]int, length+1)
	result[0] = 0

	for i := 1; i <= length; i++ {
		min := result[i-1] + 1
		for _, v := range jump[i-1] {
			if result[v+1] < min {
				min = result[v+1]
			}
		}
		result[i] = min
	}

	return result[length]
}

type trie struct {
	nodes []trieNode
}

type trieNode struct {
	next  [26]int
	final []int
	fail  int
}

func newTire(words []string) *trie {
	t := &trie{}

	dummy := t.newNode()
	root := t.newNode()

	t.build(root, words)
	t.print()
	t.mark(dummy, root)
	t.print()

	return t
}

func (t *trie) newNode() int {
	index := len(t.nodes)
	t.nodes = append(t.nodes, trieNode{})
	return index
}

func (t *trie) build(root int, words []string) {
	for _, word := range words {
		curr := root
		wordb := []byte(word)
		for _, c := range wordb {
			next := t.nodes[curr].next[c-'a']
			if next == 0 {
				next = t.newNode()
				t.nodes[curr].next[c-'a'] = next
			}
			curr = next
		}
		t.nodes[curr].final = append(t.nodes[curr].final, len(wordb))
	}
}

func (t *trie) mark(dummy, root int) {
	t.nodes[dummy].fail = dummy
	for i := range t.nodes[dummy].next {
		t.nodes[dummy].next[i] = root
	}
	t.nodes[root].fail = dummy

	q := &queueInt{}

	q.enqueue(root)

	for curr, ok := q.dequeue(); ok; curr, ok = q.dequeue() {
		fail := t.nodes[curr].fail
		for c := range t.nodes[curr].next {
			next := t.nodes[curr].next[c]
			if next == 0 {
				t.nodes[curr].next[c] = t.nodes[fail].next[c]
				continue
			}
			t.nodes[next].fail = t.nodes[fail].next[c]
			q.enqueue(next)
		}
	}

}

func (t *trie) print() {
	// for i, v := range t.nodes {
	// 	fmt.Printf("%3d : fail->%d ", i, v.fail)
	// 	for j, next := range v.next {
	// 		if next > 1 {
	// 			fmt.Printf("%s->%d ", string(byte(j+'a')), next)
	// 		}
	// 	}
	// 	fmt.Printf("final:%+v\n", v.final)
	// }
}

type queueInt struct {
	head  int
	tail  int
	array []int
}

func (q *queueInt) enqueue(v int) {
	length := len(q.array)
	if length == 0 {
		length = 1
	}
	if (q.head+1)%length == q.tail {
		newArray := make([]int, length*2+1)
		newHead := 0
		for tail := q.tail; tail != q.head; tail = (tail + 1) % length {
			newArray[newHead] = q.array[tail]
			newHead++
		}

		q.head = newHead
		q.tail = 0
		q.array = newArray
		length = len(newArray)
	}
	q.array[q.head] = v
	q.head = (q.head + 1) % length
	return
}

func (q *queueInt) dequeue() (v int, ok bool) {
	if q.tail == q.head {
		return 0, false
	}

	v = q.array[q.tail]
	q.tail = (q.tail + 1) % len(q.array)
	return v, true
}
