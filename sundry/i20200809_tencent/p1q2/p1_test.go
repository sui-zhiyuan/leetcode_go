package p1q2

import "testing"

func TestFind(t *testing.T) {
	type args struct {
		a []int
		b []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name :"case 1"
			args:{}
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Find(tt.args.a, tt.args.b); got != tt.want {
				t.Errorf("Find() = %v, want %v", got, tt.want)
			}
		})
	}
}
