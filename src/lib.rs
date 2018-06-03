#[macro_use]
extern crate matches;

mod node_mut;
pub use self::node_mut::NodeMut;

mod node;
pub use self::node::Node;

mod entry;
pub use self::entry::{Entry, OccupiedEntry, VacantEntry};

mod node_child_iter;
pub use self::node_child_iter::NodeChildIter;

mod traversal_root;
pub(crate) use self::traversal_root::TraversalRoot;

mod breadth_first_iter;
pub use self::breadth_first_iter::BreadthFirstIter;

mod depth_first_order;
pub use self::depth_first_order::DepthFirstOrder;

mod depth_first_iter;
pub use self::depth_first_iter::DepthFirstIter;

use std::mem;

/// An Eytzinger tree is an N-tree stored in an array structure.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EytzingerTree<N> {
    nodes: Vec<Option<N>>,
    max_children_per_node: usize,
    len: usize,
}

impl<N> EytzingerTree<N> {
    /// Creates a new Eytzinger tree with the specified maximum number of child nodes per parent.
    pub fn new(max_children_per_node: usize) -> Self {
        Self {
            nodes: vec![None],
            max_children_per_node,
            len: 0,
        }
    }

    /// Gets a depth first iterator over all nodes.
    pub fn depth_first_iter(&self, order: DepthFirstOrder) -> DepthFirstIter<N> {
        DepthFirstIter::new(self, self.root(), order)
    }

    /// Gets a breadth first iterator over all nodes.
    pub fn breadth_first_iter(&self) -> BreadthFirstIter<N> {
        BreadthFirstIter::new(self, self.root())
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
        self.max_children_per_node
    }

    /// Clears the Eytzinger tree, removing all nodes.
    pub fn clear(&mut self) {
        self.nodes.truncate(1);
        self.nodes[0] = None;
        self.len = 0;
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

    /// Sets the value of the root node. If the new value is `None` then all
    /// children will be removed.
    ///
    /// # Returns
    ///
    /// The new root node.
    pub fn set_root_value<V>(&mut self, new_value: V) -> NodeMut<N>
    where
        V: Into<Option<N>>,
    {
        self.set_value(0, new_value.into())
    }

    /// Gets the entry for the root node.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_eytzinger_tree::{EytzingerTree, Entry};
    ///
    /// let tree = {
    ///     let mut tree = EytzingerTree::<u32>::new(8);
    ///     tree.root_entry().or_insert(5);
    ///     tree
    /// };
    ///
    /// let root = tree.root().unwrap();
    /// assert_eq!(root.value(), &5);
    /// ```
    pub fn root_entry(&mut self) -> Entry<N> {
        self.entry(0)
    }

    fn set_child_value(&mut self, parent: usize, child: usize, new_value: Option<N>) -> NodeMut<N> {
        let child_index = self.child_index(parent, child);
        self.set_value(child_index, new_value)
    }

    fn set_value(&mut self, index: usize, new_value: Option<N>) -> NodeMut<N> {
        if index >= self.nodes.len() {
            // TODO LH use resize_default once stable
            for _ in 0..(index + 1 - self.nodes.len()) {
                self.nodes.push(None);
            }
        }

        let new_value_is_none = new_value.is_none();
        let old_value = mem::replace(&mut self.nodes[index], new_value);

        if old_value.is_some() {
            if new_value_is_none {
                self.len -= 1;

                let mut indices_to_remove = vec![];
                for child_node in DepthFirstIter::new(
                    self,
                    Some(Node { tree: self, index }),
                    DepthFirstOrder::PostOrder,
                ) {
                    indices_to_remove.push(child_node.index());
                }

                for index_to_remove in indices_to_remove {
                    let old_value = mem::replace(&mut self.nodes[index_to_remove], None);
                    if old_value.is_some() {
                        self.len -= 1
                    }
                }
            }
        } else if !new_value_is_none {
            self.len += 1;
        }

        NodeMut { tree: self, index }
    }

    fn child_index(&self, parent: usize, child: usize) -> usize {
        assert!(
            child < self.max_children_per_node,
            "the child index should be less than max_children_per_node"
        );

        (parent * self.max_children_per_node) + child + 1
    }

    fn parent_index(&self, child: usize) -> Option<usize> {
        if child == 0 {
            None
        } else {
            Some((child - 1) / self.max_children_per_node)
        }
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

    fn entry(&mut self, index: usize) -> Entry<N> {
        match self.node_mut(index) {
            Ok(node) => Entry::Occupied(OccupiedEntry { node }),
            Err(tree) => Entry::Vacant(VacantEntry { tree, index }),
        }
    }

    fn child_entry(&mut self, parent: usize, child: usize) -> Entry<N> {
        let child_index = self.child_index(parent, child);
        self.entry(child_index)
    }

    fn value(&self, index: usize) -> &Option<N> {
        &self.nodes[index]
    }

    fn value_mut(&mut self, index: usize) -> &mut Option<N> {
        &mut self.nodes[index]
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

    fn remove(&mut self, index: usize) {
        self.set_value(index, None);
    }
}

#[cfg(test)]
mod tests {
    use {DepthFirstOrder, EytzingerTree};

    #[test]
    fn root_is_none_for_empty() {
        let mut tree = EytzingerTree::<u32>::new(2);

        assert_matches!(tree.root(), None);
        assert_matches!(tree.root_mut(), None);
    }

    #[test]
    fn set_root_value_sets_root() {
        let mut tree = EytzingerTree::<u32>::new(2);

        let expected_root = Some(5);
        tree.set_root_value(expected_root);

        assert_eq!(tree.root().map(|x| *x.value()), expected_root);
        assert_eq!(tree.root_mut().map(|x| *x.value()), expected_root);
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

        let depth_first: Vec<_> = tree.depth_first_iter(DepthFirstOrder::PreOrder)
            .map(|n| n.value())
            .cloned()
            .collect();

        assert_eq!(depth_first, vec![5, 2, 1, 4, 3, 7, 8]);

        let depth_first: Vec<_> = tree.depth_first_iter(DepthFirstOrder::PostOrder)
            .map(|n| n.value())
            .cloned()
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

        let breadth_first: Vec<_> = tree.breadth_first_iter()
            .map(|n| n.value())
            .cloned()
            .collect();

        assert_eq!(breadth_first, vec![5, 2, 7, 1, 4, 8, 3]);
    }

}
