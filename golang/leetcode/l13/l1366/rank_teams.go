package l1366

import (
	"sort"
	"strings"
)

func rankTeams(votes []string) string {
	rank := [26][26]int{}
	for _, vote := range votes {
		for i := 0; i < len(vote); i++ {
			rank[vote[i]-'A'][i]++
		}
	}

	result := [26]byte{}
	for i := range result {
		result[i] = 'A' + byte(i)
	}

	sort.Slice(result[:], func(i, j int) bool {
		c1, c2 := result[i], result[j]
		for i := 0; i < 26; i++ {
			if rank[c1-'A'][i] > rank[c2-'A'][i] {
				return true
			}
			if rank[c1-'A'][i] < rank[c2-'A'][i] {
				return false
			}
		}
		return c1 < c2
	})

	r := &strings.Builder{}
	for i := 0; i < len(votes[0]); i++ {
		r.WriteByte(result[i])
	}
	return r.String()
}
