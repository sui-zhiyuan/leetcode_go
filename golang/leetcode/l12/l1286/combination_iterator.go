package l1286

// CombinationIterator ...
type CombinationIterator struct {
	bytes []byte
	n     int
	curr  []int
}

// Constructor ...
func Constructor(characters string, combinationLength int) CombinationIterator {
	n := len(characters)
	bytes := make([]byte, n)
	for i := 0; i < n; i++ {
		bytes[i] = characters[i]
	}
	curr := make([]int, combinationLength)
	for i := range curr {
		curr[i] = i
	}
	return CombinationIterator{
		bytes: bytes,
		n:     n,
		curr:  curr,
	}
}

// Next ...
func (c *CombinationIterator) Next() (s string) {
	val := make([]byte, len(c.curr))
	for i, v := range c.curr {
		val[i] = c.bytes[v]
	}
	s = string(val)

	found := -1
	for i := len(c.curr) - 1; i >= 0; i-- {
		if (i+1 >= len(c.curr) || c.curr[i+1] > c.curr[i]+1) && c.curr[i]+1 < c.n {
			found = i
			break
		}
	}

	if found == -1 {
		c.curr = nil
		return
	}

	for i, v := found, c.curr[found]+1; i < len(c.curr); i, v = i+1, v+1 {
		c.curr[i] = v
	}

	return
}

// HasNext ...
func (c *CombinationIterator) HasNext() bool {
	return c.curr != nil
}
