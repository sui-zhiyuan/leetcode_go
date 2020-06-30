package po09

// CQueue ...
type CQueue struct {
	inStack  *stack
	outStack *stack
}

// Constructor ...
func Constructor() CQueue {
	return CQueue{
		inStack:  &stack{},
		outStack: &stack{},
	}
}

// AppendTail ...
func (c *CQueue) AppendTail(value int) {
	c.inStack.push(value)
}

// DeleteHead ...
func (c *CQueue) DeleteHead() int {
	v, ok := c.outStack.pop()
	if ok {
		return v
	}

	for v, ok := c.inStack.pop(); ok; v, ok = c.inStack.pop() {
		c.outStack.push(v)
	}

	v, ok = c.outStack.pop()
	if ok {
		return v
	}
	return -1
}

type stack struct {
	top    int
	values []int
}

func (s *stack) push(v int) {
	if s.top >= len(s.values) {
		s.values = append(s.values, v)
		s.top++
		return
	}

	s.values[s.top] = v
	s.top++
}

func (s *stack) pop() (v int, ok bool) {
	if s.top == 0 {
		return 0, false
	}

	v = s.values[s.top-1]
	s.top--
	return v, true
}
