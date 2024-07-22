//package main
//
//import "fmt"

func maxArea(height []int) int {
	if height == nil {
		return 0
	}
	if len(height) < 2 {
		return 0
	}

	area := 0
	i := 0
	j := len(height) - 1
	for i < j {
		min_ := height[i]
		if min_ > height[j] {
			min_ = height[j]
		}
		area_ := (j - i) * min_
		if area_ > area {
			area = area_
		}
		//fmt.Println(i, j, area_)
		if height[j] >= height[i] {
			i++
			continue
		}
		if height[j] < height[i] {
			j--
			continue
		}
	}

	return area
}

//func main() {
//	height := []int{1, 8, 6, 2, 5, 4, 8, 3, 7}
//	fmt.Println(maxArea(height))
//}
