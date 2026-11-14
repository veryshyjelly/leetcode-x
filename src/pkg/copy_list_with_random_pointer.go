package pkg

type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	var copyMap = make(map[*Node]*Node)
	copyMap[nil] = nil

	for p := head; p != nil; p = p.Next {
		newNode := new(Node)
		newNode.Val = head.Val
		copyMap[p] = newNode
	}

	for p := head; p != nil; p = p.Next {
		copyMap[p].Next = copyMap[p.Next]
		copyMap[p].Random = copyMap[p.Random]
	}

	return copyMap[head]
}