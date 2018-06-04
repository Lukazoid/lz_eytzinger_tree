use {DepthFirstOrder, EytzingerTree, Node, NodeChildIter, TraversalRoot};

/// A depth-first iterator
#[derive(Debug)]
pub struct DepthFirstIter<'a, N>
where
    N: 'a,
{
    order: DepthFirstOrder,
    root: TraversalRoot<'a, N>,
    first_pending: Option<Node<'a, N>>,
    nodes: Vec<NodeChildIter<'a, N>>,
}

impl<'a, N> Clone for DepthFirstIter<'a, N> {
    fn clone(&self) -> Self {
        DepthFirstIter {
            order: self.order,
            root: self.root,
            first_pending: self.first_pending,
            nodes: self.nodes.clone(),
        }
    }
}

impl<'a, N> DepthFirstIter<'a, N> {
    pub(crate) fn new(
        tree: &'a EytzingerTree<N>,
        node: Option<Node<'a, N>>,
        order: DepthFirstOrder,
    ) -> Self {
        let root = if let Some(node) = node {
            TraversalRoot::Node(node)
        } else {
            TraversalRoot::Tree(tree)
        };

        Self {
            order,
            root,
            first_pending: node,
            nodes: vec![],
        }
    }

    /// Gets the order of depth-first iteration.
    pub fn order(&self) -> DepthFirstOrder {
        self.order
    }

    /// Gets the starting/root node of this iterator or `None` if there was not one. There will be
    /// no starting node for an empty Eytzinger tree.
    pub fn starting_node(&self) -> Option<Node<'a, N>> {
        self.root.starting_node()
    }

    /// Gets the tree this iterator is for.
    pub fn tree(&self) -> &'a EytzingerTree<N> {
        self.root.tree()
    }
}

impl<'a, N> Iterator for DepthFirstIter<'a, N> {
    type Item = Node<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first_node) = self.first_pending.take() {
            self.nodes.push(first_node.child_iter());

            if matches!(self.order, DepthFirstOrder::PreOrder) {
                return Some(first_node);
            }
        }

        while let Some(mut current) = self.nodes.pop() {
            if let Some(next) = current.next() {
                self.nodes.push(current);
                self.nodes.push(next.child_iter());

                if matches!(self.order, DepthFirstOrder::PreOrder) {
                    return Some(next);
                }
            } else {
                if matches!(self.order, DepthFirstOrder::PostOrder) {
                    return Some(current.node());
                }
            }
        }
        None
    }
}
