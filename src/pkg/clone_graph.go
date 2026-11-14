package pkg

//type Node struct {
//	Val       int
//	Neighbors []*Node
//}
//
//func cloneGraph(node *Node) *Node {
//	if node == nil {
//		return nil
//	}
//	visited := make([]*Node, 101)
//	newNode := new(Node)
//	newNode.Val = node.Val
//	newNode.Neighbors = make([]*Node, 0)
//	dfs(node, newNode, visited)
//	return newNode
//}
//
//func dfs(node, copy *Node, visited []*Node) {
//	visited[copy.Val] = copy
//
//	for _, n := range node.Neighbors {
//		if visited[n.Val] == nil {
//			newNode := new(Node)
//			newNode.Val = n.Val
//			newNode.Neighbors = make([]*Node, 0)
//			copy.Neighbors = append(copy.Neighbors, newNode)
//			dfs(n, newNode, visited)
//		} else {
//			copy.Neighbors = append(copy.Neighbors, visited[n.Val])
//		}
//	}
//}