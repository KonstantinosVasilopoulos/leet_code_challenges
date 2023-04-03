package main

import "fmt"

// Definition for singly-linked list
type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	// Create empty list for result
	var mergedList *ListNode = &ListNode{}
	current := mergedList

	// Iterate over both input lists
	current1, current2 := list1, list2
	for current1 != nil || current2 != nil {
		current.Next = &ListNode{}
		current = current.Next

		// Get the smaller value into the merged list
		if current1 == nil {
			current.Val = current2.Val
			current2 = current2.Next
		} else if current2 == nil {
			current.Val = current1.Val
			current1 = current1.Next
		} else if current1.Val < current2.Val {
			current.Val = current1.Val
			current1 = current1.Next
		} else {
			current.Val = current2.Val
			current2 = current2.Next
		}
	}

	return mergedList.Next
}

func printList(list *ListNode) {
	for current := list; current != nil; current = current.Next {
		fmt.Print(current.Val)
	}

	fmt.Println()
}

func main() {
	list1 := &ListNode{1, &ListNode{2, &ListNode{4, nil}}}
	list2 := &ListNode{1, &ListNode{3, &ListNode{4, nil}}}
	printList(mergeTwoLists(list1, list2))
}
