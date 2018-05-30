use std::collections::VecDeque;
use {EytzingerTree, Node, NodeChildIterator, TraversalRoot};

/// A breadth-first iterator.
#[derive(Debug)]
pub struct BreadthFirstIterator<'a, N>
where
    N: 'a,
{
    root: TraversalRoot<'a, N>,
    nodes: VecDeque<NodeChildIterator<'a, N>>,
}

impl<'a, N> Clone for BreadthFirstIterator<'a, N> {
    fn clone(&self) -> Self {
        BreadthFirstIterator {
            root: self.root,
            nodes: self.nodes.clone(),
        }
    }
}

impl<'a, N> BreadthFirstIterator<'a, N> {
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

    /// Gets the starting/root node of this iterator.
    pub fn starting_node(&self) -> Option<Node<'a, N>> {
        self.root.starting_node()
    }

    /// Gets the tree this iterator is for.
    pub fn tree(&self) -> &'a EytzingerTree<N> {
        self.root.tree()
    }
}

impl<'a, N> Iterator for BreadthFirstIterator<'a, N> {
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
}
