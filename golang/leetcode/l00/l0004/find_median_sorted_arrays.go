package l0004

import (
	"math"
)

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	count := len(nums1) + len(nums2)
	nums1 = append(nums1, math.MaxInt32)
	nums2 = append(nums2, math.MaxInt32)

	v1, v2 := find(nums1, nums2, (count-1)/2)
	if count%2 != 0 {
		return float64(v1)
	}
	return (float64(v1) + float64(v2)) / 2
}

func find(nums1 []int, nums2 []int, k int) (r1 int, r2 int) {
	// fmt.Println("+", "nums1", nums1, "nums2", nums2, "k", k)
	// defer func() {
	// 	fmt.Println("-", "nums1", nums1, "nums2", nums2, "k", k, "r", r1, r2)
	// }()
	if k == 0 {
		if nums1[0] > nums2[0] {
			nums1, nums2 = nums2, nums1
		}
		r1 = nums1[0]
		r2 = min(nums1[1], nums2[0])
		return r1, r2
	}
	p1 := min(len(nums1)-1, (k-1)/2)
	p2 := min(len(nums2)-1, (k-1)/2)
	if nums1[p1] <= nums2[p2] {
		return find(nums1[p1+1:], nums2, k-p1-1)
	}
	return find(nums1, nums2[p2+1:], k-p2-1)
}

func min(t ...int) int {
	result := t[0]
	for _, v := range t[1:] {
		if result > v {
			result = v
		}
	}
	return result
}
