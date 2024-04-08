package l5461

import (
	"strings"
)

const mod = 1_000_000_007

func numSub(s string) int {
	sb := strings.Split(s, "0")
	result := 0
	for _, v := range sb {
		l := len(v)
		result = (result + (l+1)*l/2) % mod
	}

	return result
}
