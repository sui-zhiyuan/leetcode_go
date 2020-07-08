package p0850

import (
	"sort"
)

const mod = 1_000_000_007

func rectangleArea(rectangles [][]int) int {
	xp := make([]int, 0, len(rectangles)*2)
	yp := make([]int, 0, len(rectangles)*2)

	xFound, yFound := map[int]struct{}{}, map[int]struct{}{}

	pos := [4]*[]int{&xp, &yp, &xp, &yp}
	found := [4]map[int]struct{}{xFound, yFound, xFound, yFound}
	for _, rect := range rectangles {
		for i := 0; i < 4; i++ {
			if _, ok := found[i][rect[i]]; !ok {
				found[i][rect[i]] = struct{}{}
				*pos[i] = append(*pos[i], rect[i])
			}
		}
	}

	sort.Slice(xp, func(i, j int) bool { return xp[i] < xp[j] })
	sort.Slice(yp, func(i, j int) bool { return yp[i] < yp[j] })
	//fmt.Println("pos", xp, yp)

	lenXP := len(xp)
	lenYP := len(yp)
	result := 0
	for i := 0; i < lenXP-1; i++ {
		for j := 0; j < lenYP-1; j++ {
			found := false
			for _, rect := range rectangles {
				if rect[0] <= xp[i] && rect[1] <= yp[j] && rect[2] >= xp[i+1] && rect[3] >= yp[j+1] {
					found = true
					break
				}
			}
			if found {
				//fmt.Println("found", xp[i], yp[j], xp[i+1], yp[j+1])
				area := (xp[i+1] - xp[i]) * (yp[j+1] - yp[j]) % mod
				result = (result + area) % mod
			}
		}
	}

	return result
}
