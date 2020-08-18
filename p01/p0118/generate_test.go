package p0118

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_generate(t *testing.T) {
	type args struct {
		numRows int
	}
	tests := []struct {
		name string
		args args
		want [][]int
	}{
		{
			name: "case 1",
			args: args{
				numRows: 5,
			},
			want: [][]int{
				{1},
				{1, 1},
				{1, 2, 1},
				{1, 3, 3, 1},
				{1, 4, 6, 4, 1},
			},
		},
		{
			name: "case 2",
			args: args{
				numRows: 0,
			},
			want: [][]int{},
		},
		{
			name: "case 3",
			args: args{
				numRows: 1,
			},
			want: [][]int{{1}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := generate(tt.args.numRows); !cmp.Equal(got, tt.want) {
				t.Errorf("generate() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
