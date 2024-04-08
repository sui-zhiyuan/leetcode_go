package l1406

import "testing"

func Test_stoneGameIII(t *testing.T) {
	type args struct {
		stoneValue []int
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "case 1",
			args: args{
				stoneValue: []int{1, 2, 3, 7},
			},
			want: "Bob",
		},
		{
			name: "case 2",
			args: args{
				stoneValue: []int{1, 2, 3, -9},
			},
			want: "Alice",
		},
		{
			name: "case 3",
			args: args{
				stoneValue: []int{1, 2, 3, 6},
			},
			want: "Tie",
		},
		{
			name: "case 4",
			args: args{
				stoneValue: []int{1, 2, 3, -1, -2, -3, 7},
			},
			want: "Alice",
		},
		{
			name: "case 5",
			args: args{
				stoneValue: []int{-1, -2, -3},
			},
			want: "Tie",
		},
		{
			name: "case 6",
			args: args{
				stoneValue: []int{-2},
			},
			want: "Bob",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := stoneGameIII(tt.args.stoneValue); got != tt.want {
				t.Errorf("stoneGameIII() = %v, want %v", got, tt.want)
			}
		})
	}
}
