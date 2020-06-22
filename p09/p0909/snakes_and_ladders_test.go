package p0909

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSnakesAndLadders(t *testing.T) {
	testTable := []struct {
		name  string
		board [][]int
		steps int
	}{
		{
			name: "case 0",
			board: [][]int{
				{-1, -1, -1, -1, -1, -1},
				{-1, -1, -1, -1, -1, -1},
				{-1, -1, -1, -1, -1, -1},
				{-1, 35, -1, -1, 13, -1},
				{-1, -1, -1, -1, -1, -1},
				{-1, 15, -1, -1, -1, -1},
			},
			steps: 4,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			steps := snakesAndLadders(c.board)
			assert.Equal(t, c.steps, steps)
		})
	}
}

func TestGetCoordinate(t *testing.T) {
	testTable := []struct {
		name string
		n    int
	}{
		{
			name: "case 1",
			n:    6,
		},
		{
			name: "case 2",
			n:    5,
		},
	}

	for _, c := range testTable {
		t.Run(c.name, func(t *testing.T) {
			for row := 0; row < c.n; row++ {
				for col := 0; col < c.n; col++ {
					fmt.Printf("%3d ", getCoordinate(c.n, coordinate{row, col}))
				}
				fmt.Println()
			}
		})
	}
}
