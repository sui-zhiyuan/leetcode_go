package l0443

import (
	"fmt"
)

func compress(chars []byte) int {
	currChar := byte('-')
	currCount := 0
	writePos := 0
	for _, v := range chars {
		if v == currChar {
			currCount++
			continue
		}
		writePos = write(chars, writePos, currChar, currCount)

		currChar = v
		currCount = 1
	}
	writePos = write(chars, writePos, currChar, currCount)
	return writePos
}

func write(chars []byte, writePos int, char byte, count int) (writePosOut int) {
	if count == 0 {
		return writePos
	}
	chars[writePos] = char
	writePos++
	if count > 1 {
		str := fmt.Sprint(count)
		for i := 0; i < len(str); i++ {
			chars[writePos] = str[i]
			writePos++
		}
	}
	return writePos
}
