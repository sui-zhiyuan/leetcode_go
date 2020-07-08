package p0877

type state struct {
	got  int
	loss int
}

func stoneGame(piles []int) bool {
	n := len(piles)
	buffer := make([][]state, n)
	for i := 0; i < n; i++ {
		buffer[i] = make([]state, n-i+1)
		buffer[i][1] = state{
			got:  piles[i],
			loss: 0,
		}
	}

	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			lRest := buffer[i][length-1]
			rRest := buffer[i+1][length-1]
			if lRest.loss+piles[i+length-1] > rRest.loss+piles[i] {
				buffer[i][length] = state{
					got:  lRest.loss + piles[i+length-1],
					loss: lRest.got,
				}
				continue
			}
			buffer[i][length] = state{
				got:  rRest.loss + piles[i],
				loss: rRest.got,
			}
		}
	}

	// for i := range buffer {
	// 	fmt.Println(buffer[i])
	// }

	return buffer[0][n].got > buffer[0][n].loss
}
