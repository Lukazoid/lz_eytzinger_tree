use crate::{
    traversal::{NodeChildIter, TraversalRoot},
    EytzingerTree, Node,
};
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// A breadth-first iterator.
#[derive(Debug)]
pub struct BreadthFirstIter<'a, N>
where
    N: 'a,
{
    root: TraversalRoot<'a, N>,
    nodes: VecDeque<NodeChildIter<'a, N>>,
}

impl<'a, N> Clone for BreadthFirstIter<'a, N> {
    fn clone(&self) -> Self {
        BreadthFirstIter {
            root: self.root,
            nodes: self.nodes.clone(),
        }
    }
}

impl<'a, N> BreadthFirstIter<'a, N> {
    pub(crate) fn new(tree: &'a EytzingerTree<N>, node: Option<Node<'a, N>>) -> Self {
        let mut nodes = VecDeque::new();

        let root = if let Some(node) = node {
            nodes.push_back(node.child_iter());
            TraversalRoot::Node(node)
        } else {
            TraversalRoot::Tree(tree)
        };

        Self { root, nodes }
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

impl<'a, N> Iterator for BreadthFirstIter<'a, N> {
    type Item = Node<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut current) = self.nodes.pop_front() {
            if let Some(next) = current.next() {
                self.nodes.push_front(current);
                self.nodes.push_back(next.child_iter());
            } else {
                return Some(current.node());
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree().len()))
    }
}

impl<'a, N> FusedIterator for BreadthFirstIter<'a, N> {}
