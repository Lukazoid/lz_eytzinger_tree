use std::hash::{Hash, Hasher};
use std::ops::Deref;
use {same_object, BreadthFirstIterator, DepthFirstIterator, DepthFirstOrder, EtzyngerTree,
     NodeChildIterator, NodeMut};

/// Represents a borrowed node in the Etzynger tree. This node may be used to navigate to parent or
/// child nodes.
#[derive(Debug)]
pub struct Node<'a, N>
where
    N: 'a,
{
    pub(crate) tree: &'a EtzyngerTree<N>,
    pub(crate) index: usize,
}

impl<'a, N> Hash for Node<'a, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.tree as *const EtzyngerTree<N>).hash(state);
        self.index.hash(state);
    }
}

impl<'a, N> PartialEq for Node<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        same_object(self.tree, other.tree) && self.index == other.index
    }
}
impl<'a, N> Eq for Node<'a, N> {}

impl<'a, N> Copy for Node<'a, N> {}

impl<'a, N> Clone for Node<'a, N> {
    fn clone(&self) -> Self {
        Self {
            tree: self.tree,
            index: self.index,
        }
    }
}

impl<'a, N> Node<'a, N> {
    /// Gets the Etzynger tree this node is for.
    ///
    /// # Examples
    ///
    /// ```
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
    ///     tree.set_root_value(5);
    ///     tree
    /// };
    ///
    /// let root = tree.root().unwrap();
    /// assert_eq!(root.tree(), &tree);
    /// ```
    pub fn tree(&self) -> &'a EtzyngerTree<N> {
        self.tree
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    /// Gets the value stored at this node.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
    ///     tree.set_root_value(5);
    ///     tree
    /// };
    ///
    /// let root = tree.root().unwrap();
    /// assert_eq!(root.value(), &5);
    /// ```
    pub fn value(&self) -> &'a N {
        self.tree
            .value(self.index)
            .as_ref()
            .expect("a value should exist at the index")
    }

    /// Gets the parent of this node or `None` is there was none.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
    ///     {
    ///         let mut root = tree.set_root_value(5);
    ///         root.set_child_value(2, 3);
    ///     }
    ///     tree
    /// };
    ///
    /// let root = tree.root().unwrap();
    /// let child = root.child(2).unwrap();
    /// assert_eq!(child.parent(), Some(root));
    /// ```
    pub fn parent(&self) -> Option<Node<'a, N>> {
        self.tree.parent(self.index)
    }

    /// Gets the child of this node at the specified index or `None` if there wasn't one.
    ///
    /// # Examples
    ///
    /// ```    
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let tree = {
    ///     let mut tree = EtzyngerTree::<u32>::new(8);
    ///     {
    ///         let mut root = tree.set_root_value(5);
    ///         root.set_child_value(2, 3);
    ///     }
    ///     tree
    /// };
    ///
    /// let root = tree.root().unwrap();
    /// let child = root.child(2).unwrap();
    /// assert_eq!(child.value(), &3);
    /// ```
    pub fn child(&self, index: usize) -> Option<Node<'a, N>> {
        self.tree.child(self.index, index)
    }

    /// Gets an iterator over the immediate children of this node. This only includes children
    /// for which there is a node.
    ///
    /// # Examples
    ///
    /// ```
    /// use lz_etzynger_tree::{EtzyngerTree, Node};
    ///
    /// let tree = {
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
    /// let root = tree.root().unwrap();
    /// let child_values: Vec<_> = root.child_iter().map(|n| n.value()).collect();
    /// assert_eq!(child_values, vec![&1, &3]);
    /// ```
    pub fn child_iter(&self) -> NodeChildIterator<'a, N> {
        NodeChildIterator::new(*self)
    }

    /// Gets a depth first iterator over this and all child nodes.
    pub fn depth_first_iter(&self, order: DepthFirstOrder) -> DepthFirstIterator<'a, N> {
        DepthFirstIterator::new(self.tree(), Some(*self), order)
    }

    /// Gets a breadth first iterator over this and all child nodes.
    pub fn breadth_first_iter(&self) -> BreadthFirstIterator<'a, N> {
        BreadthFirstIterator::new(self.tree(), Some(*self))
    }
}

impl<'a, N> Deref for Node<'a, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<'a, N> From<NodeMut<'a, N>> for Node<'a, N> {
    fn from(value: NodeMut<'a, N>) -> Node<'a, N> {
        Node {
            tree: value.tree,
            index: value.index,
        }
    }
}

impl<'a, N> From<&'a NodeMut<'a, N>> for Node<'a, N> {
    fn from(value: &'a NodeMut<'a, N>) -> Node<'a, N> {
        Node {
            tree: value.tree,
            index: value.index,
        }
    }
}
