package l1334

// cspell:ignore dmin, imin, jmin

func findTheCity(n int, edges [][]int, distanceThreshold int) int {
	edge := newGrid(n)
	for _, v := range edges {
		edge.set(v[0], v[1], v[2])
	}
	prev := newGrid(n)
	curr := edge
	for k := 0; k < n; k++ {
		prev, curr = curr, prev
		curr.empty()
		for i := 0; i < n; i++ {
			for j := 0; j < i; j++ {
				min, ok := 0, false
				if dmin, dok := prev.get(i, j); dok && (!ok || min > dmin) {
					min, ok = dmin, true
				}
				imin, iok := prev.get(i, k)
				jmin, jok := prev.get(k, j)
				if iok && jok && (!ok || min > imin+jmin) {
					min, ok = imin+jmin, true
				}

				if ok {
					curr.set(i, j, min)
				}
			}
		}
	}

	minCity := n + 1
	minIndex := -1
	for i := 0; i < n; i++ {
		count := 0
		for j := 0; j < n; j++ {
			if d, ok := curr.get(i, j); ok && d <= distanceThreshold {
				count++
			}
		}

		if count <= minCity {
			minCity = count
			minIndex = i
		}
	}
	return minIndex
}

type grid struct {
	arr []int
	n   int
}

func newGrid(n int) *grid {
	g := &grid{
		arr: make([]int, n*n),
		n:   n,
	}
	g.empty()
	return g
}

func (g *grid) get(a, b int) (v int, ok bool) {
	v = g.arr[a*g.n+b]
	if v < 0 {
		return 0, false
	}

	return v, true
}

func (g *grid) set(a, b, v int) {
	g.arr[a*g.n+b] = v
	g.arr[b*g.n+a] = v
}

func (g *grid) empty() {
	for i := range g.arr {
		g.arr[i] = -1
	}
	for i := 0; i < g.n; i++ {
		g.arr[i*g.n+i] = 0
	}
}
