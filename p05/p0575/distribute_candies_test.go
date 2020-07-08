package p0575

import "testing"

func Test_distributeCandies(t *testing.T) {
	type args struct {
		candies []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case 1",
			args: args{
				candies: []int{1, 1, 2, 2, 3, 3},
			},
			want: 3,
		},
		{
			name: "case 2",
			args: args{
				candies: []int{1, 1, 2, 3},
			},
			want: 2,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := distributeCandies(tt.args.candies); got != tt.want {
				t.Errorf("distributeCandies() = %v, want %v", got, tt.want)
			}
		})
	}
}
