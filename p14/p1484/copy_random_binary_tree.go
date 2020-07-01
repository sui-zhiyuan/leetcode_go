package p1484

func copyRandomBinaryTree(root *Node) *NodeCopy {
	dic := map[*Node]*NodeCopy{}
	newRoot := copyTree(dic, root)
	copyRandom(dic, root, newRoot)
	return newRoot
}

func copyTree(dic map[*Node]*NodeCopy, root *Node) *NodeCopy {
	if root == nil {
		return nil
	}

	node := &NodeCopy{
		Val:   root.Val,
		Left:  copyTree(dic, root.Left),
		Right: copyTree(dic, root.Right),
	}
	dic[root] = node
	return node
}

func copyRandom(dic map[*Node]*NodeCopy, root *Node, newRoot *NodeCopy) {
	if root == nil {
		return
	}

	if root.Random != nil {
		newRoot.Random = dic[root.Random]
	}

	copyRandom(dic, root.Left, newRoot.Left)
	copyRandom(dic, root.Right, newRoot.Right)
}
