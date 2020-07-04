package p0121

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMaxProfit(t *testing.T) {
	testTable := []struct {
		name   string
		prices []int
		profit int
	}{
		{
			name:   "case 1",
			prices: []int{7, 1, 5, 3, 6, 4},
			profit: 5,
		},
		{
			name:   "case 2",
			prices: []int{7, 6, 4, 3, 1},
			profit: 0,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			profit := maxProfit(c.prices)
			assert.Equal(t, c.profit, profit)
		})
	}
}
