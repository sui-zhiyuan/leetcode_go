package lo59

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAdd(t *testing.T) {
	type inner struct {
		input  int
		except int
	}

	cases := []struct {
		name string
		k    int
		nums []int
		val  []inner
	}{
		{
			name: "case 1",
			k:    3,
			nums: []int{4, 5, 8, 2},
			val:  []inner{{3, 4}, {5, 5}, {10, 5}, {9, 8}, {4, 8}},
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			kl := Constructor(c.k, c.nums)
			for _, v := range c.val {
				assert.Equal(t, v.except, kl.Add(v.input))
			}
		})
	}
}
