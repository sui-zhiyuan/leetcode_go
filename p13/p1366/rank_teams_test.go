package p1366

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRankTeams(t *testing.T) {
	testTable := []struct {
		name   string
		ranks  []string
		result string
	}{
		{
			name:   "case 1",
			ranks:  []string{"ABC", "ACB", "ABC", "ACB", "ACB"},
			result: "ACB",
		},
		{
			name:   "case 2",
			ranks:  []string{"WXYZ", "XYZW"},
			result: "XWYZ",
		},
		{
			name:   "case 3",
			ranks:  []string{"ZMNAGUEDSJYLBOPHRQICWFXTVK"},
			result: "ZMNAGUEDSJYLBOPHRQICWFXTVK",
		},
		{
			name:   "case 4",
			ranks:  []string{"BCA", "CAB", "CBA", "ABC", "ACB", "BAC"},
			result: "ABC",
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			result := rankTeams(c.ranks)
			assert.Equal(t, c.result, result)
		})
	}
}
