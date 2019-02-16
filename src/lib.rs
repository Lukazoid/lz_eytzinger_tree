mod eytzinger_index_calculator;
pub(crate) use self::eytzinger_index_calculator::EytzingerIndexCalculator;

mod node_mut;
pub use self::node_mut::NodeMut;

mod node;
pub use self::node::Node;

pub mod entry;
pub mod traversal;

use crate::{
    entry::{Entry, EntryMut, VacantEntry, VacantEntryMut},
    traversal::{
        BreadthFirstIter, BreadthFirstIterator, DepthFirstIter, DepthFirstIterator,
        DepthFirstOrder, NodeChildIter,
    },
};
use std::{
    cmp::PartialEq,
    hash::{Hash, Hasher},
    mem,
    ops::Range,
};

/// An Eytzinger tree is an N-tree stored in an array structure.
#[derive(Debug, Clone, Eq)]
pub struct EytzingerTree<N> {
    nodes: Vec<Option<N>>,
    index_calculator: EytzingerIndexCalculator,
    len: usize,
}

impl<N: PartialEq> PartialEq for EytzingerTree<N> {
    fn eq(&self, other: &Self) -> bool {
        self.index_calculator == other.index_calculator
            && self.len == other.len
            && self.enumerate_values().eq(other.enumerate_values())
    }
}

impl<N: Hash> Hash for EytzingerTree<N> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for indexed_value in self.enumerate_values() {
            indexed_value.hash(state);
        }
        self.index_calculator.hash(state);
        self.len.hash(state);
    }
}

impl<N> EytzingerTree<N> {
    /// Creates a new Eytzinger tree with the specified maximum number of child nodes per parent.
    ///
    /// # Returns
    ///
    /// The new Eytzinger tree.
    pub fn new(max_children_per_node: usize) -> Self {
        Self {
            nodes: vec![None],
            index_calculator: EytzingerIndexCalculator::new(max_children_per_node),
            len: 0,
        }
    }

    /// Gets a depth-first iterator over all nodes.
    pub fn depth_first_iter(&self, order: DepthFirstOrder) -> DepthFirstIter<N> {
        DepthFirstIter::new(self, self.root(), order)
    }

    /// Gets a breadth-first iterator over all nodes.
    pub fn breadth_first_iter(&self) -> BreadthFirstIter<N> {
        BreadthFirstIter::new(self, self.root())
    }

    pub fn into_depth_first_iterator(self, order: DepthFirstOrder) -> DepthFirstIterator<N> {
        DepthFirstIterator::new(self, order)
    }

    pub fn into_breadth_first_iterator(self) -> BreadthFirstIterator<N> {
        BreadthFirstIterator::new(self)
    }

    /// Gets whether the Eytzinger tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the number of nodes in the Eytzinger tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets the maximum number of children per parent node.
    pub fn max_children_per_node(&self) -> usize {
        self.index_calculator.max_children_per_node()
    }

    /// Clears the Eytzinger tree, removing all nodes.
    pub fn clear(&mut self) {
        self.remove_root_value();
    }

    /// Gets the root node, `None` if there was no root node.
    ///
    /// The root node may be set with `set_root_value`.
    pub fn root(&self) -> Option<Node<N>> {
        self.node(0)
    }

    /// Gets the mutable root node, `None` if there was no root node.
    ///
    /// The root node may be set with `set_root_value`.
    pub fn root_mut(&mut self) -> Option<NodeMut<N>> {
        self.node_mut(0).ok()
    }

    /// Sets the value of the root node. All child nodes will remain as they are.
    ///
    /// # Returns
    ///
    /// The new root node.
    pub fn set_root_value(&mut self, new_value: N) -> NodeMut<N> {
        self.set_value(0, new_value)
    }

    /// Removes the root value. This will also remove all children.
    ///
    /// # Returns
    ///
    /// The old root value if there was one.
    pub fn remove_root_value(&mut self) -> (Option<N>, VacantEntryMut<N>) {
        self.nodes.truncate(1);
        self.len = 0;
        let value = self.nodes[0].take();

        (
            value,
            VacantEntryMut {
                tree: self,
                index: 0,
            },
        )
    }

    /// Gets the mutable entry for the root node.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_eytzinger_tree::{EytzingerTree, entry::EntryMut};
    ///
    /// let tree = {
    ///     let mut tree = EytzingerTree::<u32>::new(8);
    ///     tree.root_entry_mut().or_insert(5);
    ///     tree
    /// };
    ///
    /// let root = tree.root().unwrap();
    /// assert_eq!(root.value(), &5);
    /// ```
    pub fn root_entry_mut(&mut self) -> EntryMut<N> {
        self.entry_mut(0)
    }

    /// Gets the entry for the root node.
    pub fn root_entry(&self) -> Entry<N> {
        self.entry(0)
    }

    /// Builds a new `EytzingerTree<N>` with the values mapped
    /// using the specified selector.
    pub fn map<U, F>(self, mut f: F) -> EytzingerTree<U>
    where
        F: FnMut(N) -> U,
    {
        let mut nodes = Vec::with_capacity(self.nodes.capacity());
        for node in self.nodes {
            let new_node = match node {
                Some(value) => Some(f(value)),
                None => None,
            };

            nodes.push(new_node);
        }
        EytzingerTree {
            nodes: nodes,
            index_calculator: self.index_calculator,
            len: self.len,
        }
    }

    /// Gets an iterator over each value and its index in the tree.
    fn enumerate_values(&self) -> impl Iterator<Item = (usize, &N)> {
        self.nodes
            .iter()
            .enumerate()
            .flat_map(|(i, o)| o.as_ref().map(|v| (i, v)))
    }

    fn set_child_value(&mut self, parent: usize, child: usize, new_value: N) -> NodeMut<N> {
        let child_index = self.child_index(parent, child);
        self.set_value(child_index, new_value)
    }

    fn ensure_size(&mut self, index: usize) {
        // TODO LH Use resize_default once stable
        let desired_len = index + 1;
        if let Some(additional) = desired_len.checked_sub(self.nodes.len()) {
            self.nodes.reserve(additional);

            for _ in 0..additional {
                self.nodes.push(None);
            }
        }
    }

    fn remove(&mut self, index: usize) -> Option<N> {
        if index >= self.nodes.len() {
            return None;
        }

        let indices_to_remove: Vec<_> = self
            .node(index)?
            .depth_first_iter(DepthFirstOrder::PostOrder)
            .skip(1)
            .map(|n| n.index)
            .collect();

        let old_value = self.nodes[index].take();

        if old_value.is_some() {
            self.len -= 1;
        }

        for index_to_remove in indices_to_remove {
            let removed_child_value = self.nodes[index_to_remove].take();
            if removed_child_value.is_some() {
                self.len -= 1
            }
        }

        old_value
    }

    fn split_off(&mut self, index: usize) -> EytzingerTree<N> {
        let mut new_tree = EytzingerTree::new(self.max_children_per_node());

        // get all of the indexes which should be moved out of the source tree
        let indexes_to_move = self.node(index).map(|n| {
            n.depth_first_iter(DepthFirstOrder::PreOrder)
                .map(|n| n.index)
                .collect::<Vec<_>>()
        });

        if let Some(indexes_to_move) = indexes_to_move {
            let mut indexes_to_move_iter = indexes_to_move.into_iter();

            if let Some(index_to_move) = indexes_to_move_iter.next() {
                let new_root_value = self.nodes[index_to_move]
                    .take()
                    .expect("there should be a value at the index returned by the iterator");

                self.len -= 1;

                let mut new_node = new_tree.set_root_value(new_root_value);

                // this is used to determine if we need to move up a level
                let mut previous_parent = self.parent_index(index_to_move);

                for index_to_move in indexes_to_move_iter {
                    let value_to_move = self.nodes[index_to_move]
                        .take()
                        .expect("there should be a value at the index returned by the iterator");

                    self.len -= 1;

                    let current_parent = self
                        .parent_index(index_to_move)
                        .expect("the root should only ever be the first node in the iterator");

                    if let Some(mut previous_parent) = previous_parent {
                        while current_parent <= previous_parent {
                            new_node = new_node.to_parent().ok().expect(
                                "the root should only ever be the first node in the iterator",
                            );
                            previous_parent = self.parent_index(previous_parent).unwrap();
                        }
                    }

                    previous_parent = Some(current_parent);

                    let child_offset = index_to_move - self.child_index(current_parent, 0);
                    new_node = new_node
                        .to_child_entry_mut(child_offset)
                        .or_insert(value_to_move);
                }
            }
        }

        new_tree
    }

    fn set_value(&mut self, index: usize, new_value: N) -> NodeMut<N> {
        self.ensure_size(index);

        let old_value = mem::replace(&mut self.nodes[index], Some(new_value));

        if old_value.is_none() {
            self.len += 1;
        }

        NodeMut { tree: self, index }
    }

    fn child_index(&self, parent_index: usize, child_offset: usize) -> usize {
        self.index_calculator
            .child_index(parent_index, child_offset)
    }

    fn parent_index(&self, child_index: usize) -> Option<usize> {
        self.index_calculator.parent_index(child_index)
    }

    fn child_indexes(&self, parent_index: usize) -> Range<usize> {
        self.index_calculator.child_indexes(parent_index)
    }

    fn node(&self, index: usize) -> Option<Node<N>> {
        if let Some(Some(_)) = self.nodes.get(index) {
            Some(Node { tree: self, index })
        } else {
            None
        }
    }

    fn node_mut(&mut self, index: usize) -> Result<NodeMut<N>, &mut Self> {
        if let Some(Some(_)) = self.nodes.get_mut(index) {
            Ok(NodeMut {
                tree: self,
                index: index,
            })
        } else {
            Err(self)
        }
    }

    fn entry(&self, index: usize) -> Entry<N> {
        match self.node(index) {
            Some(node) => Entry::Occupied(node),
            None => Entry::Vacant(VacantEntry { tree: self, index }),
        }
    }

    fn entry_mut(&mut self, index: usize) -> EntryMut<N> {
        match self.node_mut(index) {
            Ok(node) => EntryMut::Occupied(node),
            Err(tree) => EntryMut::Vacant(VacantEntryMut { tree, index }),
        }
    }

    fn child_entry(&self, parent: usize, child: usize) -> Entry<N> {
        let child_index = self.child_index(parent, child);
        self.entry(child_index)
    }

    fn child_entry_mut(&mut self, parent: usize, child: usize) -> EntryMut<N> {
        let child_index = self.child_index(parent, child);
        self.entry_mut(child_index)
    }

    fn value(&self, index: usize) -> Option<&Option<N>> {
        self.nodes.get(index)
    }

    fn value_mut(&mut self, index: usize) -> Option<&mut Option<N>> {
        self.nodes.get_mut(index)
    }

    fn parent(&self, child: usize) -> Option<Node<N>> {
        let parent_index = self.parent_index(child)?;
        self.node(parent_index)
    }

    fn parent_mut(&mut self, child: usize) -> Result<NodeMut<N>, &mut Self> {
        if let Some(parent_index) = self.parent_index(child) {
            self.node_mut(parent_index)
        } else {
            Err(self)
        }
    }

    fn child(&self, parent: usize, child: usize) -> Option<Node<N>> {
        let child_index = self.child_index(parent, child);
        self.node(child_index)
    }

    fn child_mut(&mut self, parent: usize, child: usize) -> Result<NodeMut<N>, &mut Self> {
        let child_index = self.child_index(parent, child);
        self.node_mut(child_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::{DepthFirstOrder, EytzingerTree};
    use matches::assert_matches;

    #[test]
    fn root_is_none_for_empty() {
        let mut tree = EytzingerTree::<u32>::new(2);

        assert_matches!(tree.root(), None);
        assert_matches!(tree.root_mut(), None);
    }

    #[test]
    fn set_root_value_sets_root() {
        let mut tree = EytzingerTree::<u32>::new(2);

        let expected_root = 5;
        tree.set_root_value(expected_root);

        assert_eq!(tree.root().map(|x| *x.value()).unwrap(), expected_root);
        assert_eq!(tree.root_mut().map(|x| *x.value()).unwrap(), expected_root);
    }

    #[test]
    fn depth_first_iter_returns_empty_for_empty_tree() {
        let tree = EytzingerTree::<u32>::new(2);

        assert_matches!(
            tree.depth_first_iter(DepthFirstOrder::PostOrder).next(),
            None
        )
    }

    #[test]
    fn depth_first_iter_returns_depth_first() {
        let mut tree = EytzingerTree::<u32>::new(2);
        {
            let mut root = tree.set_root_value(5);
            {
                let mut left = root.set_child_value(0, 2);

                left.set_child_value(0, 1);
                let mut left_right = left.set_child_value(1, 4);
                left_right.set_child_value(0, 3);
            }
            {
                let mut right = root.set_child_value(1, 7);
                right.set_child_value(1, 8);
            }
        }

        assert_eq!(tree.len(), 7);

        let depth_first: Vec<_> = tree
            .depth_first_iter(DepthFirstOrder::PreOrder)
            .map(|n| n.value())
            .cloned()
            .collect();

        assert_eq!(depth_first, vec![5, 2, 1, 4, 3, 7, 8]);

        let depth_first: Vec<_> = tree
            .depth_first_iter(DepthFirstOrder::PostOrder)
            .map(|n| n.value())
            .cloned()
            .collect();

        assert_eq!(depth_first, vec![1, 3, 4, 2, 8, 7, 5]);
    }

    #[test]
    fn into_depth_first_iterator_pre_order() {
        let mut tree = EytzingerTree::<u32>::new(2);
        {
            let mut root = tree.set_root_value(5);
            {
                let mut left = root.set_child_value(0, 2);

                left.set_child_value(0, 1);
                let mut left_right = left.set_child_value(1, 4);
                left_right.set_child_value(0, 3);
            }
            {
                let mut right = root.set_child_value(1, 7);
                right.set_child_value(1, 8);
            }
        }

        assert_eq!(tree.len(), 7);

        let depth_first: Vec<_> = tree
            .into_depth_first_iterator(DepthFirstOrder::PreOrder)
            .collect();

        assert_eq!(depth_first, vec![5, 2, 1, 4, 3, 7, 8]);
    }

    #[test]
    fn into_depth_first_iterator_post_order() {
        let mut tree = EytzingerTree::<u32>::new(2);
        {
            let mut root = tree.set_root_value(5);
            {
                let mut left = root.set_child_value(0, 2);

                left.set_child_value(0, 1);
                let mut left_right = left.set_child_value(1, 4);
                left_right.set_child_value(0, 3);
            }
            {
                let mut right = root.set_child_value(1, 7);
                right.set_child_value(1, 8);
            }
        }

        assert_eq!(tree.len(), 7);

        let depth_first: Vec<_> = tree
            .into_depth_first_iterator(DepthFirstOrder::PostOrder)
            .collect();

        assert_eq!(depth_first, vec![1, 3, 4, 2, 8, 7, 5]);
    }

    #[test]
    fn breadth_first_iter_returns_empty_for_empty_tree() {
        let tree = EytzingerTree::<u32>::new(2);

        assert_matches!(tree.breadth_first_iter().next(), None)
    }

    #[test]
    fn breadth_first_iter_returns_breadth_first() {
        let mut tree = EytzingerTree::<u32>::new(2);
        {
            let mut root = tree.set_root_value(5);
            {
                let mut left = root.set_child_value(0, 2);

                left.set_child_value(0, 1);
                let mut left_right = left.set_child_value(1, 4);
                left_right.set_child_value(0, 3);
            }
            {
                let mut right = root.set_child_value(1, 7);
                right.set_child_value(1, 8);
            }
        }

        assert_eq!(tree.len(), 7);

        let breadth_first: Vec<_> = tree
            .breadth_first_iter()
            .map(|n| n.value())
            .cloned()
            .collect();

        assert_eq!(breadth_first, vec![5, 2, 7, 1, 4, 8, 3]);
    }

    #[test]
    fn into_breadth_first_iterator_returns_breadth_first() {
        let mut tree = EytzingerTree::<u32>::new(2);
        {
            let mut root = tree.set_root_value(5);
            {
                let mut left = root.set_child_value(0, 2);

                left.set_child_value(0, 1);
                let mut left_right = left.set_child_value(1, 4);
                left_right.set_child_value(0, 3);
            }
            {
                let mut right = root.set_child_value(1, 7);
                right.set_child_value(1, 8);
            }
        }

        assert_eq!(tree.len(), 7);

        let breadth_first: Vec<_> = tree.into_breadth_first_iterator().collect();

        assert_eq!(breadth_first, vec![5, 2, 7, 1, 4, 8, 3]);
    }
}
