package p1232

func checkStraightLine(coordinates [][]int) bool {
	length := len(coordinates)
	if length < 3 {
		return true
	}

	x1, y1, x2, y2 := coordinates[0][0], coordinates[0][1], coordinates[1][0], coordinates[1][1]

	for _, coordinate := range coordinates[2:] {
		xc, yc := coordinate[0], coordinate[1]

		if (x2-x1)*(yc-y1) != (xc-x1)*(y2-y1) {
			return false
		}
	}

	return true
}
