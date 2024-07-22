//package main
//
//import "fmt"

func Partition(nums []int, i, j int) int {
	m := i - 1
	v := nums[j]
	for i < j {
		if nums[i] < v {
			m = m + 1
			nums[i], nums[m] = nums[m], nums[i]
		}
		i++
	}
	m = m + 1
	nums[i], nums[m] = nums[m], nums[i]
	nums[m] = v
	return m
}

func QuickSort(nums []int, i, j int) []int {
	if i >= j {
		return nums
	}
	part := Partition(nums, i, j)
	QuickSort(nums, i, part-1)
	QuickSort(nums, part+1, j)
	return nums
}

func threeSum(nums []int) [][]int {
	sumArr := make([][]int, 0)
	nums = QuickSort(nums, 0, len(nums)-1)
	for i := 0; i <= len(nums)-3; i++ {
		left := i + 1
		right := len(nums) - 1
		for left < right {
			sum := nums[i] + nums[left] + nums[right]
			if sum == 0 {
				arr := []int{nums[i], nums[left], nums[right]}
				sumArr = append(sumArr, arr)
				for left < right {
					if nums[left+1] == nums[left] {
						left++
						continue
					}
					break
				}
				left++
				continue
			}
			if sum > 0 {
				for left < right {
					if nums[right-1] == nums[right] {
						right--
						continue
					}
					break
				}
				right--
				continue
			}
			if sum < 0 {
				for left < right {
					if nums[left+1] == nums[left] {
						left++
						continue
					}
					break
				}
				left++
				continue
			}
		}
		for i <= len(nums)-3 {
			if nums[i] == nums[i+1] {
				i++
				continue
			}
			break
		}
	}
	return sumArr
}

//func main() {
//	arr := []int{-2, -1, 0, 2, 3}
//	fmt.Println(threeSum(arr))
//}
