package pkg

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	if head == nil {
		return false
	}
	var tortoise, hare = head, head

	for tortoise.Next != nil && hare.Next != nil && hare.Next.Next != nil {
		tortoise = tortoise.Next
		hare = hare.Next.Next
		if tortoise == hare {
			return true
		}
	}

	return false
}