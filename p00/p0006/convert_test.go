package p0006

import "testing"

func Test_convert(t *testing.T) {
	type args struct {
		s       string
		numRows int
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			name: "case 1",
			args: args{
				s:       "LEETCODEISHIRING",
				numRows: 3,
			},
			want: "LCIRETOESIIGEDHN",
		},
		{
			name: "case 2",
			args: args{
				s:       "LEETCODEISHIRING",
				numRows: 4,
			},
			want: "LDREOEIIECIHNTSG",
		},
		{
			name: "case 3",
			args: args{
				s:       "LEETCODEISHIRING",
				numRows: 1,
			},
			want: "LEETCODEISHIRING",
		},
		{
			name: "case 4",
			args: args{
				s:       "LEETCODEISHIRING",
				numRows: 2,
			},
			want: "LECDIHRNETOESIIG",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := convert(tt.args.s, tt.args.numRows); got != tt.want {
				t.Errorf("convert() = %v, want %v", got, tt.want)
			}
		})
	}
}
