//package main
//
//import (
//	"encoding/json"
//	"fmt"
//)
//
//// Definition for singly-linked list.
//type ListNode struct {
//	Val  int
//	Next *ListNode
//}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	node1 := list1
	node2 := list2
	var merged *ListNode

	step := func(node1, node2 **ListNode) *ListNode {
		var merged *ListNode
		// choose the beginning
		if *node1 == nil && *node2 == nil {
			return nil
		}
		if *node1 != nil && *node2 != nil {
			if (*node1).Val <= (*node2).Val {
				merged = *node1
				*node1 = (*node1).Next
			} else {
				merged = *node2
				*node2 = (*node2).Next
			}
		} else if *node1 == nil {
			merged = *node2
			*node2 = (*node2).Next
		} else if *node2 == nil {
			merged = *node1
			*node1 = (*node1).Next
		}
		return merged
	}
	merged = step(&node1, &node2)
	head := merged
	if merged == nil {
		return head
	}

	for node1 != nil || node2 != nil {
		choose := step(&node1, &node2)
		if choose == nil {
			break
		}
		merged.Next = choose
		merged = merged.Next
	}

	return head
}

//func main() {
//	list1 := &ListNode{Val: 0}
//	var list2 *ListNode
//
//	head := mergeTwoLists(list1, list2)
//	inf, _ := json.Marshal(head)
//	fmt.Println(string(inf))
//}
