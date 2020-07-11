package p1256

import "testing"

func Test_encode(t *testing.T) {
	type args struct {
		num int
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "case 1",
			args: args{num: 23},
			want: "1000",
		},
		{
			name: "case 2",
			args: args{num: 107},
			want: "101100",
		},
		{
			name: "case 3",
			args: args{num: 0},
			want: "",
		},
		{
			name: "case 4",
			args: args{num: 1},
			want: "0",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := encode(tt.args.num); got != tt.want {
				t.Errorf("encode() = %v, want %v", got, tt.want)
			}
		})
	}
}
