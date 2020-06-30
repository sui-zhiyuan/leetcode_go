package p5432

import (
	"sort"
)

func average(salary []int) float64 {
	sort.Slice(salary, func(i, j int) bool {
		return salary[i] < salary[j]
	})

	total := 0
	for _, v := range salary[1 : len(salary)-1] {
		total += v
	}

	return float64(total) / float64(len(salary)-2)
}
