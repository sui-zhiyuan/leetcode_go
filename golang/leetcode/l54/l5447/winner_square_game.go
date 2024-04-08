package l5447

func winnerSquareGame(n int) bool {
	status := make([]bool, n+1)
	status[0] = false
	for i := 1; i <= n; i++ {
		for k := 1; ; k++ {
			kq := k * k
			if kq > i {
				break
			}
			if !status[i-kq] {
				status[i] = true
				break
			}
		}
	}
	return status[n]
}
