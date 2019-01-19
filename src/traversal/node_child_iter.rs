use crate::Node;
use std::iter::FusedIterator;

/// An iterator over the immediate children of a single node.
#[derive(Debug)]
pub struct NodeChildIter<'a, N>
where
    N: 'a,
{
    node: Node<'a, N>,
    child_offset: usize,
}

impl<'a, N> Clone for NodeChildIter<'a, N> {
    fn clone(&self) -> Self {
        NodeChildIter {
            node: self.node,
            child_offset: self.child_offset,
        }
    }
}

impl<'a, N> NodeChildIter<'a, N> {
    pub(crate) fn new(node: Node<'a, N>) -> Self {
        Self {
            node,
            child_offset: 0,
        }
    }

    /// Gets the node this iterator is for.
    pub fn node(&self) -> Node<'a, N> {
        self.node
    }
}

impl<'a, N> Iterator for NodeChildIter<'a, N> {
    type Item = Node<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.child_offset < self.node.tree().max_children_per_node() {
            let next_child = self.node.child(self.child_offset);
            self.child_offset += 1;
            if let Some(next_child) = next_child {
                return Some(next_child);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.node.tree().max_children_per_node()))
    }
}

impl<'a, N> FusedIterator for NodeChildIter<'a, N> {}
