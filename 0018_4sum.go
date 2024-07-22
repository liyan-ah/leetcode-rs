//package main
//
//import "fmt"

func Partition(nums []int, i, j int) int {
	m := i - 1
	v := nums[j]
	for i < j {
		if nums[i] < v {
			m++
			nums[m], nums[i] = nums[i], nums[m]
		}
		i++
	}
	m++
	nums[m], nums[i] = nums[i], nums[m]
	nums[m] = v
	return m
}

func QuickSort(nums []int, i, j int) []int {
	if nums == nil || len(nums) == 0 || i >= j {
		return nums
	}
	m := Partition(nums, i, j)
	QuickSort(nums, i, m-1)
	QuickSort(nums, m+1, j)
	return nums
}

func threeSum(nums []int, begin, exist, target int) [][]int {
	sumArr := make([][]int, 0)
	for i := begin; i <= len(nums)-3; i++ {
		left := i + 1
		right := len(nums) - 1
		for left < right {
			sum := nums[i] + nums[left] + nums[right] + exist
			if sum == target {
				arr := []int{exist, nums[i], nums[left], nums[right]}
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
			if sum > target {
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
			if sum < target {
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

func fourSum(nums []int, target int) [][]int {
	nums = QuickSort(nums, 0, len(nums)-1)
	sumArr := make([][]int, 0)
	for i := 0; i <= len(nums)-4; i++ {
		exist := nums[i]
		arr := threeSum(nums, i+1, exist, target)
		sumArr = append(sumArr, arr...)
		for i <= len(nums)-4 {
			if nums[i+1] == nums[i] {
				i++
				continue
			}
			break
		}
	}
	return sumArr
}

//func main() {
//	nums := []int{2, 2, 2, 2, 2}
//	fmt.Println(fourSum(nums, 8))
//}
