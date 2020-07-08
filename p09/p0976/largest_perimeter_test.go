package p0976

import "testing"

func Test_largestPerimeter(t *testing.T) {
	type args struct {
		A []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				A: []int{2, 1, 2},
			},
			want: 5,
		},
		{
			name: "case 2",
			args: args{
				A: []int{1, 2, 1},
			},
			want: 0,
		},
		{
			name: "case 3",
			args: args{
				A: []int{3, 2, 3, 4},
			},
			want: 10,
		},
		{
			name: "case 4",
			args: args{
				A: []int{3, 6, 2, 3},
			},
			want: 8,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := largestPerimeter(tt.args.A); got != tt.want {
				t.Errorf("largestPerimeter() = %v, want %v", got, tt.want)
			}
		})
	}
}
