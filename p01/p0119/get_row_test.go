package p0119

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func Test_getRow(t *testing.T) {
	type args struct {
		rowIndex int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "case 1",
			args: args{
				rowIndex: 5,
			},
			want: []int{1, 5, 10, 10, 5, 1},
		},
		{
			name: "case 2",
			args: args{
				rowIndex: 0,
			},
			want: []int{1},
		},
		{
			name: "case 3",
			args: args{
				rowIndex: 1,
			},
			want: []int{1, 1},
		},
		{
			name: "case 4",
			args: args{
				rowIndex: 3,
			},
			want: []int{1, 3, 3, 1},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := getRow(tt.args.rowIndex); !cmp.Equal(got, tt.want) {
				t.Errorf("getRow() = %v, want %v\ndiff=%v", got, tt.want, cmp.Diff(got, tt.want))
			}
		})
	}
}
