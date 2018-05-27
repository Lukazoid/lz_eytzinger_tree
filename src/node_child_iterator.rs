use Node;

/// An iterator over the immediate children of a single node.
#[derive(Debug)]
pub struct NodeChildIterator<'a, N>
where
    N: 'a,
{
    node: Node<'a, N>,
    index: usize,
}

impl<'a, N> Clone for NodeChildIterator<'a, N> {
    fn clone(&self) -> Self {
        NodeChildIterator {
            node: self.node,
            index: self.index,
        }
    }
}

impl<'a, N> NodeChildIterator<'a, N> {
    pub(crate) fn new(node: Node<'a, N>) -> Self {
        Self { node, index: 0 }
    }

    /// Gets the node this iterator is for.
    pub fn node(&self) -> Node<'a, N> {
        self.node
    }
}

impl<'a, N> Iterator for NodeChildIterator<'a, N> {
    type Item = Node<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.node.tree().max_children_per_node() {
            let next_child = self.node.child(self.index);
            self.index += 1;
            if let Some(next_child) = next_child {
                return Some(next_child);
            }
        }
        None
    }
}
