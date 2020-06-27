package p0271

import (
	"bytes"
)

// Codec ...
type Codec struct {
}

const maxLength = 0b1111_1111_1111_1111
const maxByte = 0b1_0000_0000
const nextFalg = 0b_0000_0001

// Encode encodes a list of strings to a single string.
func (codec *Codec) Encode(strs []string) string {
	ps := make([]part, 0, len(strs))
	for _, s := range strs {
		length := len(s)
		for {
			currLen := length
			next := false
			if currLen > maxLength {
				currLen = maxLength
				next = true
			}
			length = length - currLen
			ps = append(ps, part{uint16(currLen), next})
			if length == 0 {
				break
			}
		}
	}

	count := uint32(len(ps))
	buffer := bytes.NewBuffer(nil)
	for i := 0; i < 4; i++ {
		buffer.WriteByte(byte(count % maxByte))
		count = count / maxByte
	}

	for _, v := range ps {
		bs := encodePart(v)
		buffer.Write(bs[:])
	}

	for _, s := range strs {
		buffer.WriteString(s)
	}

	return buffer.String()
}

// Decode decodes a single string to a list of strings.
func (codec *Codec) Decode(strs string) []string {
	buffer := bytes.NewBuffer([]byte(strs))
	count := 0
	site := 1
	var err error
	for i := 0; i < 4; i++ {
		b, err := buffer.ReadByte()
		if err != nil {
			panic(err)
		}
		count = count + int(b)*site
		site = site * maxByte
	}

	ps := make([]part, 0, count)
	for i := 0; i < count; i++ {
		bs := [4]byte{}
		for j := range bs {
			bs[j], err = buffer.ReadByte()
			if err != nil {
				panic(err)
			}
		}
		ps = append(ps, decodePart(bs))
	}

	strLen := 0
	result := make([]string, 0, count)
	for _, v := range ps {
		strLen += int(v.length)
		if v.next {
			continue
		}
		bs := make([]byte, strLen)
		for i := range bs {
			bs[i], err = buffer.ReadByte()
			if err != nil {
				panic(err)
			}
		}
		result = append(result, string(bs))
		strLen = 0
	}

	return result
}

type part struct {
	length uint16
	next   bool
}

func encodePart(p part) [4]byte {
	r := [4]byte{}
	r[0] = byte(p.length % maxByte)
	r[1] = byte(p.length / maxByte)
	r[2] = 0
	if p.next {
		r[3] = r[3] | nextFalg
	}
	r[3] = 0
	return r
}

func decodePart(r [4]byte) part {
	p := part{}
	p.length = uint16(r[1])*maxByte + uint16(r[0])
	p.next = r[3]&nextFalg == nextFalg
	return p
}
