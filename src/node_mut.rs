use std::ops::{Deref, DerefMut};
use crate::{BreadthFirstIter, DepthFirstIter, DepthFirstOrder, Entry, EytzingerTree, Node, NodeChildIter};

/// Represents a borrowed node in the Eytzinger tree. This node may be used mutate this node's value
/// and child nodes.
#[derive(Debug)]
pub struct NodeMut<'a, N>
where
    N: 'a,
{
    pub(crate) tree: &'a mut EytzingerTree<N>,
    pub(crate) index: usize,
}

impl<'a, N> NodeMut<'a, N> {
    /// Gets the Eytzinger tree this node is for.
    pub fn tree(&self) -> &EytzingerTree<N> {
        self.tree
    }

    /// Gets the value stored at this node.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_eytzinger_tree::{EytzingerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EytzingerTree::<u32>::new(8);
    ///     tree.set_root_value(5);
    ///     tree
    /// };
    ///
    /// let root = tree.root_mut().unwrap();
    /// assert_eq!(root.value(), &5);
    /// ```
    pub fn value(&self) -> &N {
        self.as_node().value()
    }

    /// Gets the mutable value stored at this node.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_eytzinger_tree::{EytzingerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EytzingerTree::<u32>::new(8);
    ///     tree.set_root_value(5);
    ///     tree
    /// };
    ///
    /// let mut root = tree.root_mut().unwrap();
    /// *root.value_mut() = 8;
    /// assert_eq!(root.value(), &8);
    /// ```
    pub fn value_mut(&mut self) -> &mut N {
        self.tree
            .value_mut(self.index)
            .and_then(|v| v.as_mut())
            .expect("a value should exist at the index")
    }

    /// Gets the mutable value stored at this node.
    ///
    /// This differs from `value_mut` in that it takes ownership of the current node and the value
    /// is lifetime bound to the tree and not to the current node.
    pub fn into_value_mut(self) -> &'a mut N {
        self.tree
            .value_mut(self.index)
            .and_then(|v| v.as_mut())
            .expect("a value should exist at the index")
    }

    /// Gets the parent of this node or `None` is there was none.
    pub fn parent(&self) -> Option<Node<N>> {
        self.as_node().parent()
    }

    /// Gets the mutable paret of this node or `None` if there wasn't one.
    ///
    /// This differs from `parent_mut` in that it takes ownership of the current node and is
    /// lifetime bound to the tree and not to the current node.
    pub fn to_parent(self) -> Result<Self, Self> {
        let tree = self.tree;
        match tree.parent_mut(self.index) {
            Ok(parent) => Ok(parent),
            Err(tree) => Err(Self {
                tree,
                index: self.index,
            }),
        }
    }

    /// Gets the child of this node at the specified index or `None` if there wasn't one.
    pub fn child(&self, index: usize) -> Option<Node<N>> {
        self.as_node().child(index)
    }

    /// Gets the mutable child of this node at the specified index or `None` if there wasn't one.
    pub fn child_mut(&mut self, index: usize) -> Option<NodeMut<N>> {
        self.tree.child_mut(self.index, index).ok()
    }

    /// Gets the mutable child of this node at the specified index or `None` if there wasn't one.
    ///
    /// This differs from `child_mut` in that it takes ownership of the current node and is
    /// lifetime bound to the tree and not to the current node.
    pub fn to_child(self, index: usize) -> Result<Self, Self> {
        let tree = self.tree;
        match tree.child_mut(self.index, index) {
            Ok(child) => Ok(child),
            Err(tree) => Err(Self {
                tree,
                index: self.index,
            }),
        }
    }

    /// Sets the value of the child at the specified index.
    ///
    /// # Returns
    ///
    /// The new mutable child.
    pub fn set_child_value(&mut self, index: usize, new_value: N) -> NodeMut<N> {
        self.tree.set_child_value(self.index, index, new_value)
    }

    /// Removes the child value at the specified child index. This will also remove all children of
    /// the specified child.
    ///
    /// # Returns
    ///
    /// The old child value if there was one.
    pub fn remove_child_value(&mut self, index: usize) -> Option<N> {
        self.child_entry(index).remove()
    }

    /// Gets the child entry of this node at the specified index. This node is not consumed in the
    /// process so the child entry is lifetime bound to this node.
    pub fn child_entry(&mut self, index: usize) -> Entry<N> {
        self.tree.child_entry(self.index, index)
    }

    /// Gets the child entry of this node at the specified index.
    ///
    /// This differs from `child_entry` in that it takes ownership of the current node and the
    /// entry is lifetime bound to the tree and not to the current node.
    pub fn to_child_entry(self, index: usize) -> Entry<'a, N> {
        self.tree.child_entry(self.index, index)
    }

    /// Removes this node from the tree.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_eytzinger_tree::{EytzingerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EytzingerTree::<u32>::new(8);
    ///     tree.set_root_value(5);
    ///     tree
    /// };
    /// {
    ///     let mut root = tree.root_mut().unwrap();
    ///     root.remove();
    /// }
    /// assert_eq!(tree.root(), None);
    /// ```
    pub fn remove(self) -> N {
        self.tree
            .remove(self.index)
            .expect("there should be a value at the node index")
    }

    /// Gets a view of this mutable node as an immutable node. The resulting node is lifetime bound
    /// to this node so the immutable node may not outlife this mutable node.
    pub fn as_node<'b>(&'b self) -> Node<'b, N> {
        Node {
            tree: self.tree,
            index: self.index,
        }
    }

    /// Gets an iterator over the immediate children of this node. This only includes children
    /// for which there is a node.
    ///
    /// # Examples
    ///
    /// ```
    /// use lz_eytzinger_tree::{EytzingerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EytzingerTree::<u32>::new(8);
    ///     {
    ///         let mut root = tree.set_root_value(5);
    ///         root.set_child_value(0, 1);
    ///         root.set_child_value(2, 3);
    ///
    ///     }
    ///     tree
    /// };
    ///
    /// let root = tree.root_mut().unwrap();
    /// let child_values: Vec<_> = root.child_iter().map(|n| n.value()).collect();
    /// assert_eq!(child_values, vec![&1, &3]);
    /// ```
    pub fn child_iter<'b>(&'b self) -> NodeChildIter<'b, N> {
        self.as_node().child_iter()
    }

    /// Gets a depth-first iterator over this and all child nodes.
    pub fn depth_first_iter<'b>(&'b self, order: DepthFirstOrder) -> DepthFirstIter<'b, N> {
        self.as_node().depth_first_iter(order)
    }

    /// Gets a breadth-first iterator over this and all child nodes.
    pub fn breadth_first_iter<'b>(&'b self) -> BreadthFirstIter<'b, N> {
        self.as_node().breadth_first_iter()
    }

    pub fn split_off(self) -> EytzingerTree<N> {
        self.tree.split_off(self.index)
    }
}

impl<'a, N> Deref for NodeMut<'a, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<'a, N> DerefMut for NodeMut<'a, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::EytzingerTree;

    #[test]
    fn split_off() {
        let mut tree = EytzingerTree::new(2);

        let split_off = {
            let mut child = tree.root_entry()
                .or_insert(10)
                .to_child_entry(0)
                .or_insert(5);
            child
                .child_entry(0)
                .or_insert(4)
                .child_entry(0)
                .or_insert(1);
            child.child_entry(1).or_insert(8);

            child.split_off()
        };

        let mut expected_remaining = EytzingerTree::new(2);
        {
            expected_remaining.root_entry().or_insert(10);
        }

        let mut expected_split_off = EytzingerTree::new(2);
        {
            let mut root = expected_split_off.root_entry().or_insert(5);

            root.child_entry(0).or_insert(4).child_entry(0).or_insert(1);
            root.child_entry(1).or_insert(8);
        }

        assert_eq!(tree, expected_remaining);
        assert_eq!(split_off, expected_split_off);
    }

}
