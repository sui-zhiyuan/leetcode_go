package p0735

func asteroidCollision(asteroids []int) []int {
	top := 1
	curr := 1
	n := len(asteroids)
	for curr < n {
		if asteroids[curr] > 0 || top == 0 || asteroids[top-1] < 0 {
			asteroids[top] = asteroids[curr]
			top++
			curr++
			continue
		}

		if asteroids[top-1] < -asteroids[curr] {
			top--
			continue
		}

		if asteroids[top-1] > -asteroids[curr] {
			curr++
			continue
		}

		top--
		curr++

	}
	return asteroids[:top]
}
