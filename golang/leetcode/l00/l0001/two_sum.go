package l0001

type node struct {
	value    int
	location int
}

func twoSum(nums []int, target int) []int {
	data := make([]node, 0, len(nums))
	for i, v := range nums {
		data = append(data, node{value: v, location: i})
	}
	quickSort(data)
	i, j := 0, len(data)-1
	for {
		switch {
		case i == j:
			return []int{}
		case data[i].value+data[j].value > target:
			j--
		case data[i].value+data[j].value < target:
			i++
		case data[i].value+data[j].value == target:
			return []int{data[i].location, data[j].location}
		}
	}
	// unreachable path
}

func quickSort(data []node) {
	z := data[(0+len(data)-1)/2].value
	i, j := 0, len(data)-1
	for i <= j {
		for data[i].value < z {
			i++
		}

		for data[j].value > z {
			j--
		}
		if i <= j {
			data[i], data[j] = data[j], data[i]
			i++
			j--
		}
	}
	if j > 0 {
		quickSort(data[0 : j+1])
	}
	if i < len(data)-1 {
		quickSort(data[i:])
	}
}
