package p0038

import (
	"bytes"
	"fmt"
)

func countAndSay(n int) string {
	value := []byte("1")
	for i := 1; i < n; i++ {
		value = next(value)
	}
	return string(value)
}

func next(value []byte) []byte {
	buffer := &bytes.Buffer{}
	curr := byte('0')
	count := 0
	for _, v := range value {
		if v != curr {
			if count > 0 {
				buffer.WriteString(fmt.Sprintf("%d", count))
				buffer.WriteByte(curr)
			}
			curr = v
			count = 0
		}
		count++
	}
	buffer.WriteString(fmt.Sprintf("%d", count))
	buffer.WriteByte(curr)
	return buffer.Bytes()
}
