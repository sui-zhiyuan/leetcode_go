package li0806

const (
	stateUnknown = iota
	stateStart
	stateMove
	stateRecover
	stateFinal
)

// cspell:ignore hanota

func hanota(A []int, B []int, C []int) []int {
	length := len(A)
	towers := [3][]int{A, B, C}

	nodes := make([]node, length)
	curr := 0
	nodes[curr] = node{
		source: 0,
		target: 2,
		plate:  length,
		state:  stateStart,
	}

	for curr >= 0 {
		if nodes[curr].plate == 1 {
			src, tar := nodes[curr].source, nodes[curr].target
			//fmt.Println("moving_1", towers[src][len(towers[src])-1], "from", src, "to", tar)
			p := towers[src][len(towers[src])-1]
			towers[src] = towers[src][:len(towers[src])-1]
			towers[tar] = append(towers[tar], p)
			curr--
			continue
		}

		if nodes[curr].state == stateStart {
			nodes[curr].state = stateMove
			src, tar := nodes[curr].source, nodes[curr].target
			middle := 3 - src - tar
			length := nodes[curr].plate - 1
			curr++
			nodes[curr] = node{
				source: src,
				target: middle,
				plate:  length,
				state:  stateStart,
			}
			continue
		}

		if nodes[curr].state == stateMove {
			src, tar := nodes[curr].source, nodes[curr].target
			// fmt.Println("moving_2", towers[src][len(towers[src])-1], "from", src, "to", tar)
			p := towers[src][len(towers[src])-1]
			towers[src] = towers[src][:len(towers[src])-1]
			towers[tar] = append(towers[tar], p)
			nodes[curr].state = stateRecover
			continue

		}

		if nodes[curr].state == stateRecover {
			nodes[curr].state = stateFinal
			src, tar := nodes[curr].source, nodes[curr].target
			middle := 3 - src - tar
			length := nodes[curr].plate - 1
			curr++
			nodes[curr] = node{
				source: middle,
				target: tar,
				plate:  length,
				state:  stateStart,
			}
			continue
		}

		if nodes[curr].state == stateFinal {
			curr--
		}
	}

	return towers[2]
}

type node struct {
	source int
	target int
	plate  int
	state  int
}
