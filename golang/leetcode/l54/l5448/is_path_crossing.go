package l5448

func isPathCrossing(path string) bool {
	curr := site{0, 0}
	arrive := map[site]struct{}{curr: {}}

	for i := 0; i < len(path); i++ {
		v := path[i]
		next := site{
			row: curr.row + dir[v].row,
			col: curr.col + dir[v].col,
		}

		if _, ok := arrive[next]; ok {
			return true
		}

		arrive[next] = struct{}{}
		curr = next
	}

	return false
}

var dir = map[byte]site{'N': {-1, 0}, 'S': {1, 0}, 'E': {0, 1}, 'W': {0, -1}}

type site struct {
	row int
	col int
}
