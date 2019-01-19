use crate::{
    BreadthFirstIter, DepthFirstIter, DepthFirstOrder, EytzingerTree, Node, NodeChildIter, NodeMut,
};

/// An entry can be used to reference a node in an Eytzinger tree. The node may or may not have a
/// value.
#[derive(Debug)]
pub enum Entry<'a, N>
where
    N: 'a,
{
    /// When the entry references a node which exists with a value.
    Occupied(NodeMut<'a, N>),

    /// When the entry references a non-existent node.
    Vacant(VacantEntry<'a, N>),
}

/// For an entry where node does not exist.
#[derive(Debug)]
pub struct VacantEntry<'a, N>
where
    N: 'a,
{
    pub(crate) tree: &'a mut EytzingerTree<N>,
    pub(crate) index: usize,
}

impl<'a, N> VacantEntry<'a, N> {
    /// Gets the Eytzinger tree this entry is for.
    pub fn tree(&self) -> &EytzingerTree<N> {
        self.tree
    }

    /// Gets the parent of this entry or `None` is there was none (i.e. if this entry is for the root).
    pub fn parent(&self) -> Option<Node<N>> {
        self.tree.parent(self.index)
    }

    /// Gets the mutable parent of this entry or itself is there was none (i.e. if this entry is for the root).
    pub fn to_parent(self) -> Result<NodeMut<'a, N>, Self> {
        let index = self.index;
        self.tree
            .parent_mut(index)
            .map_err(|tree| VacantEntry { tree, index })
    }

    /// Inserts a value at the referenced position.
    ///
    /// # Returns
    ///
    /// The new mutable node.
    pub fn insert(self, value: N) -> NodeMut<'a, N> {
        self.tree.set_value(self.index, value)
    }

    /// Inserts a value at the referenced position.
    ///
    /// # Returns
    ///
    /// The new mutable node.
    pub fn insert_with<F>(self, value_factory: F) -> NodeMut<'a, N>
    where
        F: FnOnce() -> N,
    {
        self.tree.set_value(self.index, value_factory())
    }
}

impl<'a, N> Entry<'a, N> {
    /// Gets the Eytzinger tree this entry is for.
    pub fn tree(&self) -> &EytzingerTree<N> {
        match self {
            Entry::Occupied(node) => node.tree(),
            Entry::Vacant(vacant_entry) => vacant_entry.tree(),
        }
    }

    /// Gets the parent of this entry or `None` is there was none (i.e. if this entry is for the root).
    pub fn parent(&self) -> Option<Node<N>> {
        match self {
            Entry::Occupied(node) => node.parent(),
            Entry::Vacant(vacant_entry) => vacant_entry.parent(),
        }
    }

    /// Gets the mutable parent of this entry or itself is there was none (i.e. if this entry is for the root).
    pub fn to_parent(self) -> Result<NodeMut<'a, N>, Self> {
        match self {
            Entry::Occupied(node) => node.to_parent().map_err(|node| Entry::Occupied(node)),
            Entry::Vacant(vacant_entry) => vacant_entry
                .to_parent()
                .map_err(|vacant_entry| Entry::Vacant(vacant_entry)),
        }
    }

    /// Inserts a value at the referenced position if there is no node already there.
    ///
    /// # Returns
    ///
    /// The mutable node, this may be new or may have already existed.
    pub fn or_insert(self, value: N) -> NodeMut<'a, N> {
        match self {
            Entry::Occupied(node) => node,
            Entry::Vacant(vacant) => vacant.insert(value),
        }
    }

    /// Inserts a value at the referenced position if there is no node already there.
    ///
    /// # Returns
    ///
    /// The mutable node, this may be new or may have already existed.
    pub fn or_insert_with<F>(self, value_factory: F) -> NodeMut<'a, N>
    where
        F: FnOnce() -> N,
    {
        match self {
            Entry::Occupied(node) => node,
            Entry::Vacant(vacant) => vacant.insert_with(value_factory),
        }
    }

    /// Modifies the value (if one exists).
    ///
    /// # Returns
    ///
    /// The entry.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut N),
    {
        match self {
            Entry::Occupied(mut node) => {
                f(node.value_mut());
                Entry::Occupied(node)
            }
            entry @ Entry::Vacant(_) => entry,
        }
    }

    /// Removes the node if one existed.
    ///
    /// # Returns
    ///
    /// The removed value if there was a node and the now vacant entry.
    pub fn remove(self) -> (Option<N>, VacantEntry<'a, N>) {
        match self {
            Entry::Occupied(node) => {
                let (removed_value, vacant_entry) = node.remove();

                (Some(removed_value), vacant_entry)
            }
            Entry::Vacant(vacant_entry) => (None, vacant_entry),
        }
    }

    /// Gets the node this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The node if there was one, `None` otherwise.
    pub fn node(&self) -> Option<Node<N>> {
        match self {
            Entry::Occupied(node) => Some(node.as_node()),
            Entry::Vacant(_) => None,
        }
    }

    /// Gets the mutable node this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable node if there was one, `None` otherwise.
    pub fn node_mut(self) -> Option<NodeMut<'a, N>> {
        match self {
            Entry::Occupied(node) => Some(node),
            Entry::Vacant(_) => None,
        }
    }

    /// Gets an iterator over the immediate children of this node. This only includes children
    /// for which there is a node.
    pub fn child_iter(&self) -> EntryIter<NodeChildIter<N>> {
        match self {
            Entry::Occupied(node) => EntryIter::Occupied(node.child_iter()),
            Entry::Vacant(_) => EntryIter::Vacant,
        }
    }

    /// Gets a depth-first iterator over this and all child nodes.
    pub fn depth_first_iter(&self, order: DepthFirstOrder) -> EntryIter<DepthFirstIter<N>> {
        match self {
            Entry::Occupied(node) => EntryIter::Occupied(node.depth_first_iter(order)),
            Entry::Vacant(_) => EntryIter::Vacant,
        }
    }

    /// Gets a breadth-first iterator over this and all child nodes.
    pub fn breadth_first_iter(&self) -> EntryIter<BreadthFirstIter<N>> {
        match self {
            Entry::Occupied(node) => EntryIter::Occupied(node.breadth_first_iter()),
            Entry::Vacant(_) => EntryIter::Vacant,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntryIter<I> {
    Occupied(I),
    Vacant,
}

impl<I> Iterator for EntryIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EntryIter::Occupied(iterator) => iterator.next(),
            EntryIter::Vacant => None,
        }
    }
}
