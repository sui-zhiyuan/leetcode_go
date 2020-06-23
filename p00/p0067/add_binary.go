package p0067

import (
	"strings"
)

func addBinary(a string, b string) string {
	length := len(a)
	if len(b) > length {
		length = len(b)
	}
	length++

	result := make([]bool, length)
	la, lb, lr := len(a)-1, len(b)-1, length-1
	upper := false
	for la >= 0 || lb >= 0 {
		va, vb := false, false
		if la >= 0 {
			va = a[la] == '1'
			la--
		}
		if lb >= 0 {
			vb = b[lb] == '1'
			lb--
		}

		result[lr] = va != vb != upper
		lr--
		upper = va && vb || va && upper || vb && upper
	}

	if upper {
		result[lr] = true
		lr--
	}

	builder := &strings.Builder{}
	for _, v := range result[lr+1:] {
		if v {
			builder.WriteByte('1')
		} else {
			builder.WriteByte('0')
		}
	}

	return builder.String()
}
