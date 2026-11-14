package pkg

func detectCycle(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	var tortoise, hare = head, head

	for tortoise.Next != nil && hare.Next != nil && hare.Next.Next != nil {
		tortoise = tortoise.Next
		hare = hare.Next.Next
		if tortoise == hare {
			tortoise = head
			for tortoise != hare {
				tortoise = tortoise.Next
				hare = hare.Next
			}
			return hare
		}
	}

	return nil
}