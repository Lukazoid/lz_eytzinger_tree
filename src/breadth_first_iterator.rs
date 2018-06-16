use EytzingerTree;
use std::collections::VecDeque;
use std::iter::{ExactSizeIterator, FusedIterator};
use std::ops::Range;

/// A breadth-first iterator which returns owned values.
#[derive(Debug, Clone)]
pub struct BreadthFirstIterator<N> {
    tree: EytzingerTree<N>,
    pending_nodes: VecDeque<Range<usize>>,
}

impl<N> BreadthFirstIterator<N> {
    pub(crate) fn new(tree: EytzingerTree<N>) -> Self {
        let mut pending_nodes = VecDeque::new();

        if tree.root().is_some() {
            pending_nodes.push_back(0..1);
        }
        Self {
            tree,
            pending_nodes,
        }
    }
}

impl<N> Iterator for BreadthFirstIterator<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut current) = self.pending_nodes.pop_front() {
            if let Some(next) = current.next() {
                if current.len() > 0 {
                    // If there are still more remaining nodes at this level put them to the front
                    // of the queue
                    self.pending_nodes.push_front(current);
                }

                if let Some(next_value) = self.tree.value_mut(next).and_then(|v| v.take()) {
                    self.pending_nodes.push_back(self.tree.child_indexes(next));
                    return Some(next_value);
                }
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

impl<N> FusedIterator for BreadthFirstIterator<N> {}
