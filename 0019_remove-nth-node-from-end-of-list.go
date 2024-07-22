//package main
//
//import (
//	"encoding/json"
//	"fmt"
//)

//Definition for singly-linked list.
//type ListNode struct {
//	Val  int
//	Next *ListNode
//}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	var fast, slow *ListNode
	i := 0
	fast = head
	if fast == nil || fast.Next == nil {
		return nil
	}
	for i = 1; ; i++ {
		if i == n {
			slow = head
		}
		if fast == nil || fast.Next == nil {
			break
		}
		fast = fast.Next
		if i > n {
			slow = slow.Next
		}
	}
	if i == n {
		return head.Next
	}
	slow.Next = slow.Next.Next
	return head
}

//func main() {
//	head := &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3,
//		Next: &ListNode{Val: 4, Next: &ListNode{Val: 5}}}}}
//	//head := &ListNode{Val: 1, Next: &ListNode{Val: 2}}
//	//head := &ListNode{Val: 1}
//	h := removeNthFromEnd(head, 5)
//	if h == nil {
//		fmt.Println(h)
//		return
//	}
//
//	hByt, _ := json.Marshal(h)
//	fmt.Println(string(hByt))
//}
