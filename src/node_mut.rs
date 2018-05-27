use std::ops::{Deref, DerefMut};
use {BreadthFirstIterator, DepthFirstIterator, DepthFirstOrder, EtzyngerTree, Node,
     NodeChildIterator};

/// Represents a borrowed node in the Etzynger tree. This node may be used mutate this node's value
/// and child nodes.
#[derive(Debug)]
pub struct NodeMut<'a, N>
where
    N: 'a,
{
    pub(crate) tree: &'a mut EtzyngerTree<N>,
    pub(crate) index: usize,
}

impl<'a, N> NodeMut<'a, N> {
    /// Gets the Etzynger tree this node is for.
    pub fn tree(&self) -> &EtzyngerTree<N> {
        self.tree
    }

    /// Gets the value stored at this node.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
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
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
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
            .as_mut()
            .expect("a value should exist at the index")
    }

    /// Gets the parent of this node or `None` is there was none.
    pub fn parent(&self) -> Option<Node<N>> {
        self.as_node().parent()
    }

    /// Gets the mutable parent of this node or `None` if there wasn't one.
    pub fn parent_mut(&mut self) -> Option<NodeMut<N>> {
        self.tree.parent_mut(self.index).ok()
    }

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
    pub fn set_child_value<V>(&mut self, index: usize, new_value: V) -> NodeMut<N>
    where
        V: Into<Option<N>>,
    {
        self.tree
            .set_child_value(self.index, index, new_value.into())
    }

    /// Removes this node from the tree.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
    ///     tree.set_root_value(5);
    ///     tree
    /// };
    /// {
    ///     let mut root = tree.root_mut().unwrap();
    ///     root.remove();
    /// }
    /// assert_eq!(tree.root(), None);
    /// ```
    pub fn remove(self) {
        self.tree.remove(self.index)
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
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let mut tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
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
    pub fn child_iter<'b>(&'b self) -> NodeChildIterator<'b, N> {
        self.as_node().child_iter()
    }

    pub fn depth_first_iter<'b>(&'b self, order: DepthFirstOrder) -> DepthFirstIterator<'b, N> {
        self.as_node().depth_first_iter(order)
    }

    pub fn breadth_first_iter<'b>(&'b self) -> BreadthFirstIterator<'b, N> {
        self.as_node().breadth_first_iter()
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
