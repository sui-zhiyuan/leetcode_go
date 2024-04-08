package l5463

import (
	"fmt"
	"math"
)

func getMinDistSum(positions [][]int) float64 {
	currX := float64(50)
	currY := float64(50)
	step := float64(1)

	dx := float64(0)
	dy := float64(0)

	for i := 0; i < 1000; i++ {
		fmt.Println(currX, currY, sumPath(positions, currX, currY))
		newdx, newdy, ok := caculateDirection(positions, currX, currY)
		if !ok {
			newdx, newdy = 0.0001, 0.0001
		}
		currX, currY = currX-newdx*step, currY-newdy*step
		if newdx*dx < 0 || newdy*dy < 0 {
			step = step * 0.8
		}
		dx, dy = newdx, newdy
	}

	return sumPath(positions, currX, currY)
}

func sumPath(positions [][]int, x, y float64) float64 {
	sum := float64(0)

	for _, v := range positions {
		sum += math.Sqrt((x-float64(v[0]))*(x-float64(v[0])) + (y-float64(v[1]))*(y-float64(v[1])))
	}

	return sum
}

func caculateDirection(positions [][]int, x, y float64) (dx, dy float64, ok bool) {
	dx = 0
	dy = 0
	for _, v := range positions {
		s := math.Sqrt((x-float64(v[0]))*(x-float64(v[0])) + (y-float64(v[1]))*(y-float64(v[1])))
		if s == 0 {
			return 0, 0, false
		}
		dx = dx + (x-float64(v[0]))/s
		dy = dy + (y-float64(v[1]))/s
	}
	return dx, dy, true
}
