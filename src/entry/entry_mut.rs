use crate::{
    entry::{Entry, EntryIter, VacantEntry},
    BreadthFirstIter, DepthFirstIter, DepthFirstOrder, EytzingerTree, Node, NodeChildIter, NodeMut,
};
use matches::matches;

/// An entry can be used to reference a node in an Eytzinger tree. The node may or may not have a
/// value.
#[derive(Debug)]
pub enum EntryMut<'a, N>
where
    N: 'a,
{
    /// When the entry references a node which exists with a value.
    Occupied(NodeMut<'a, N>),

    /// When the entry references a non-existent node.
    Vacant(VacantEntryMut<'a, N>),
}

impl<'a, N> From<NodeMut<'a, N>> for EntryMut<'a, N> {
    fn from(value: NodeMut<'a, N>) -> Self {
        EntryMut::Occupied(value)
    }
}

/// For an entry where node does not exist.
#[derive(Debug)]
pub struct VacantEntryMut<'a, N>
where
    N: 'a,
{
    pub(crate) tree: &'a mut EytzingerTree<N>,
    pub(crate) index: usize,
}

impl<'a, N> VacantEntryMut<'a, N> {
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
            .map_err(|tree| VacantEntryMut { tree, index })
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

    /// Gets an immutable view of this mutable vacant entry.
    pub fn as_vacant_entry(&self) -> VacantEntry<N> {
        VacantEntry {
            tree: self.tree,
            index: self.index,
        }
    }
}

impl<'a, N> EntryMut<'a, N> {
    /// Gets the Eytzinger tree this entry is for.
    pub fn tree(&self) -> &EytzingerTree<N> {
        match self {
            EntryMut::Occupied(node) => node.tree(),
            EntryMut::Vacant(vacant_entry) => vacant_entry.tree(),
        }
    }

    pub(crate) fn index(&self) -> usize {
        match self {
            EntryMut::Occupied(node) => node.index,
            EntryMut::Vacant(vacant_entry) => vacant_entry.index,
        }
    }

    /// Gets the parent of this entry or `None` is there was none (i.e. if this entry is for the root).
    pub fn parent(&self) -> Option<Node<N>> {
        match self {
            EntryMut::Occupied(node) => node.parent(),
            EntryMut::Vacant(vacant_entry) => vacant_entry.parent(),
        }
    }

    /// Gets the mutable parent of this entry or itself is there was none (i.e. if this entry is for the root).
    pub fn to_parent(self) -> Result<NodeMut<'a, N>, Self> {
        match self {
            EntryMut::Occupied(node) => node.to_parent().map_err(|node| EntryMut::Occupied(node)),
            EntryMut::Vacant(vacant_entry) => vacant_entry
                .to_parent()
                .map_err(|vacant_entry| EntryMut::Vacant(vacant_entry)),
        }
    }

    /// Inserts a value at the referenced position if there is no node already there.
    ///
    /// # Returns
    ///
    /// The mutable node, this may be new or may have already existed.
    pub fn or_insert(self, value: N) -> NodeMut<'a, N> {
        match self {
            EntryMut::Occupied(node) => node,
            EntryMut::Vacant(vacant) => vacant.insert(value),
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
            EntryMut::Occupied(node) => node,
            EntryMut::Vacant(vacant) => vacant.insert_with(value_factory),
        }
    }

    /// Inserts a value at the referenced position if there is no node already there.
    pub fn or_insert_mut(&mut self, value: N) -> &mut Self {
        take_mut::take(self, |x| match x {
            occupied @ EntryMut::Occupied(_) => occupied,
            EntryMut::Vacant(vacant) => EntryMut::Occupied(vacant.insert(value)),
        });

        self
    }

    /// Inserts a value at the referenced position if there is no node already there.
    pub fn or_insert_with_mut<F>(&mut self, value_factory: F) -> &mut Self
    where
        F: FnOnce() -> N,
    {
        take_mut::take(self, |x| match x {
            occupied @ EntryMut::Occupied(_) => occupied,
            EntryMut::Vacant(vacant) => EntryMut::Occupied(vacant.insert_with(value_factory)),
        });

        self
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
            EntryMut::Occupied(mut node) => {
                f(node.value_mut());
                EntryMut::Occupied(node)
            }
            vacant @ EntryMut::Vacant(_) => vacant,
        }
    }

    /// Modifies the value (if one exists).
    ///
    /// # Returns
    ///
    /// The entry.
    pub fn and_modify_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut N),
    {
        match self {
            EntryMut::Occupied(node) => {
                f(node.value_mut());
            }
            EntryMut::Vacant(_) => {}
        }
        self
    }

    /// Sets the value at the specified entry.
    ///
    /// # Returns
    ///
    /// The old value and the current node.
    pub fn set_value(self, value: N) -> (Option<N>, NodeMut<'a, N>) {
        let (old_value, vacant_entry) = self.remove();

        let node = vacant_entry.insert(value);

        (old_value, node)
    }

    /// Sets the value at the specified entry.
    ///
    /// # Returns
    ///
    /// The old value.
    pub fn set_value_mut(&mut self, value: N) -> Option<N> {
        let old_value = self.remove_mut();

        self.or_insert_mut(value);

        old_value
    }

    /// Removes the node if one existed.
    ///
    /// # Returns
    ///
    /// The removed value if there was a node and the now vacant entry.
    pub fn remove(self) -> (Option<N>, VacantEntryMut<'a, N>) {
        match self {
            EntryMut::Occupied(node) => {
                let (removed_value, vacant_entry) = node.remove();
                (Some(removed_value), vacant_entry)
            }
            EntryMut::Vacant(vacant_entry) => (None, vacant_entry),
        }
    }

    /// Removes the node if one existed.
    ///
    /// # Returns
    ///
    /// The removed value if there was a node.
    pub fn remove_mut(&mut self) -> Option<N> {
        let mut value_to_return = None;
        take_mut::take(self, |x| match x {
            EntryMut::Occupied(node) => {
                let (removed_value, vacant_entry) = node.remove();
                value_to_return = Some(removed_value);

                EntryMut::Vacant(vacant_entry)
            }
            vacant @ EntryMut::Vacant(_) => vacant,
        });

        value_to_return
    }

    /// Gets the node this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The node if there was one, `None` otherwise.
    pub fn node(&self) -> Option<Node<N>> {
        Entry::from(self).node()
    }

    /// Gets the mutable value this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable value if there was one, `None` otherwise.
    pub fn value_mut(&mut self) -> Option<&mut N> {
        self.node_mut().map(|n| n.value_mut())
    }

    /// Gets the mutable value this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable value if there was one, `None` otherwise.
    pub fn into_value_mut(self) -> Option<&'a mut N> {
        self.to_node_mut().map(|n| n.into_value_mut())
    }

    /// Gets the mutable node this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable node if there was one, `None` otherwise.
    pub fn node_mut(&mut self) -> Option<&mut NodeMut<'a, N>> {
        match self {
            EntryMut::Occupied(node) => Some(node),
            EntryMut::Vacant(_) => None,
        }
    }

    /// Gets the mutable node this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable node if there was one, `None` otherwise.
    pub fn to_node_mut(self) -> Option<NodeMut<'a, N>> {
        match self {
            EntryMut::Occupied(node) => Some(node),
            EntryMut::Vacant(_) => None,
        }
    }

    /// Gets the mutable vacant entry this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable vacant entry if there was one, `None` otherwise.
    pub fn vacant_entry_mut(&mut self) -> Option<&mut VacantEntryMut<'a, N>> {
        match self {
            EntryMut::Occupied(_) => None,
            EntryMut::Vacant(vacant_entry) => Some(vacant_entry),
        }
    }

    /// Gets the mutable vacant entry this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable vacant entry if there was one, `None` otherwise.
    pub fn to_vacant_entry_mut(self) -> Option<VacantEntryMut<'a, N>> {
        match self {
            EntryMut::Occupied(_) => None,
            EntryMut::Vacant(vacant_entry) => Some(vacant_entry),
        }
    }

    /// Gets an immutable view of this mutable entry.
    pub fn as_entry(&self) -> Entry<N> {
        match self {
            EntryMut::Occupied(node) => Entry::Occupied(node.as_node()),
            EntryMut::Vacant(vacant_entry) => Entry::Vacant(vacant_entry.as_vacant_entry()),
        }
    }

    /// Gets an iterator over the immediate children of this node. This only includes children
    /// for which there is a node.
    pub fn child_iter(&self) -> EntryIter<NodeChildIter<N>> {
        match self {
            EntryMut::Occupied(node) => EntryIter::Occupied(node.child_iter()),
            EntryMut::Vacant(_) => EntryIter::Vacant,
        }
    }

    /// Gets a depth-first iterator over this and all child nodes.
    pub fn depth_first_iter(&self, order: DepthFirstOrder) -> EntryIter<DepthFirstIter<N>> {
        match self {
            EntryMut::Occupied(node) => EntryIter::Occupied(node.depth_first_iter(order)),
            EntryMut::Vacant(_) => EntryIter::Vacant,
        }
    }

    /// Gets a breadth-first iterator over this and all child nodes.
    pub fn breadth_first_iter(&self) -> EntryIter<BreadthFirstIter<N>> {
        match self {
            EntryMut::Occupied(node) => EntryIter::Occupied(node.breadth_first_iter()),
            EntryMut::Vacant(_) => EntryIter::Vacant,
        }
    }

    /// Gets whether this entry is occupied.
    pub fn is_occupied(&self) -> bool {
        matches!(self, EntryMut::Occupied(_))
    }

    /// Gets whether this entry is vacant..
    pub fn is_vacant(&self) -> bool {
        matches!(self, EntryMut::Vacant(_))
    }
}
