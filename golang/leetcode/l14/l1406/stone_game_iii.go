package l1406

func stoneGameIII(stoneValue []int) string {
	length := len(stoneValue)
	statusSpace := make([]status, length)
	for i := length - 1; i >= 0; i-- {
		sum := 0
		best := status{
			got: -10000,
		}
		for j := 0; j <= 2; j++ {
			curr := status{}
			if i+j < length {
				sum += stoneValue[i+j]
			}
			if i+j+1 < length {
				curr = statusSpace[i+j+1]
			}

			if curr.loss+sum > best.got {
				best.got = curr.loss + sum
				best.loss = curr.got
			}
		}
		statusSpace[i] = best
	}

	if statusSpace[0].got > statusSpace[0].loss {
		return "Alice"
	}
	if statusSpace[0].got == statusSpace[0].loss {
		return "Tie"
	}
	return "Bob"
}

type status struct {
	got  int
	loss int
}
