package pkg

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	var n, m uint = 0, 0
	for p := headA; p != nil; p = p.Next {
		n += 1
	}
	for p := headB; p != nil; p = p.Next {
		m += 1
	}
	for i := m; i < n; i++ {
		headA = headA.Next
	}
	for i := n; i < m; i++ {
		headB = headB.Next
	}

	for headA != nil && headB != nil && headA != headB {
		headA = headA.Next
		headB = headB.Next
	}

	if headA == nil || headB == nil {
		return nil
	}

	return headA
}