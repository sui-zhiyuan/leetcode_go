package p5455

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_minInteger(t *testing.T) {
	type args struct {
		num string
		k   int
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "case 1",
			args: args{
				num: "4321",
				k:   4,
			},
			want: "1342",
		},
		{
			name: "case 2",
			args: args{
				num: "100",
				k:   1,
			},
			want: "010",
		},
		{
			name: "case 3",
			args: args{
				num: "36789",
				k:   1000,
			},
			want: "36789",
		},
		{
			name: "case 4",
			args: args{
				num: "22",
				k:   22,
			},
			want: "22",
		},
		{
			name: "case 5",
			args: args{
				num: "9438957234785635408",
				k:   23,
			},
			want: "0345989723478563548",
		},
		{
			name: "case 6",
			args: args{
				num: "294984148179",
				k:   11,
			},
			want: "124498948179",
		},
		{
			name: "case 7",
			args: args{
				num: "858957035719081",
				k:   2,
			},
			want: "588597035719081",
		},
		{
			name: "case 8",
			args: args{
				num: "55456497803213206850845",
				k:   10,
			},
			want: "05455649783213206850845",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := minInteger(tt.args.num, tt.args.k)
			assert.Equal(t, tt.want, got)
		})
	}
}

func Test_binaryIndexedTree(t *testing.T) {
	n := 9
	element := make([]int, n)
	for i := 0; i < n; i++ {
		element[i] = i + 1
	}

	tree := newBinaryIndexedTree(element)
	fmt.Println(tree.nums)
	fmt.Println(tree.sums)
	for i := 0; i <= n; i++ {
		fmt.Printf(" [0 ~ %d = %d] ", i, tree.sum(i))
	}
	fmt.Println()
	for i := 0; i <= n; i++ {
		for j := i; j <= n; j++ {
			fmt.Printf(" [%d ~ %d = %d] ", i, j, tree.sumBetween(i, j))
		}
		fmt.Println()
	}
}
