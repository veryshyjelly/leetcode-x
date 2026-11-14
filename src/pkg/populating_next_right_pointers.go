package pkg

//
//type Node struct {
//    Val int
//    Left *Node
//    Right *Node
//    Next *Node
//}
//
//func connect(root *Node) *Node {
//    connect_and_inside(root.Left, root.Right)
//    connect(root.Left)
//    connect(root.Right)
//}
//
//func connect_and_inside(left, right *Node) {
//    if left == nil {
//        return
//    }
//    left.Next = right;
//    connect_and_inside(left.Right, right.Left)
//}