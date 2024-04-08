package l5432

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAverage(t *testing.T) {
	testTable := []struct {
		name    string
		salary  []int
		average float64
	}{
		{
			name:    "case 1",
			salary:  []int{4000, 3000, 1000, 2000},
			average: 2500,
		},
		{
			name:    "case 2",
			salary:  []int{1000, 2000, 3000},
			average: 2000,
		},
		{
			name:    "case 3",
			salary:  []int{6000, 5000, 4000, 3000, 2000, 1000},
			average: 3500,
		},
		{
			name:    "case 4",
			salary:  []int{8000, 9000, 2000, 3000, 6000, 1000},
			average: 4750,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			average := average(c.salary)
			assert.Equal(t, c.average, average)
		})
	}
}
