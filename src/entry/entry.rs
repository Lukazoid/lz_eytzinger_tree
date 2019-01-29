use crate::{
    entry::{EntryMut, VacantEntryMut},
    BreadthFirstIter, DepthFirstIter, DepthFirstOrder, EytzingerTree, Node, NodeChildIter, NodeMut,
};
use matches::matches;

#[derive(Debug)]
pub enum Entry<'a, N>
where
    N: 'a,
{
    /// When the entry references a node which exists with a value.
    Occupied(Node<'a, N>),

    /// When the entry references a non-existent node.
    Vacant(VacantEntry<'a, N>),
}

impl<'a, N> Copy for Entry<'a, N> {}

impl<'a, N> Clone for Entry<'a, N> {
    fn clone(&self) -> Self {
        match self {
            Entry::Occupied(node) => Entry::Occupied(*node),
            Entry::Vacant(vacant_entry) => Entry::Vacant(*vacant_entry),
        }
    }
}

impl<'a, 'b, N> From<&'b EntryMut<'a, N>> for Entry<'b, N> {
    fn from(value: &'b EntryMut<'a, N>) -> Self {
        match value {
            EntryMut::Occupied(node) => Entry::Occupied(node.as_node()),
            EntryMut::Vacant(vacant_entry) => Entry::Vacant(vacant_entry.into()),
        }
    }
}

impl<'a, N> From<EntryMut<'a, N>> for Entry<'a, N> {
    fn from(value: EntryMut<'a, N>) -> Self {
        match value {
            EntryMut::Occupied(node) => Entry::Occupied(node.into()),
            EntryMut::Vacant(vacant_entry) => Entry::Vacant(vacant_entry.into()),
        }
    }
}

impl<'a, N> From<NodeMut<'a, N>> for Entry<'a, N> {
    fn from(value: NodeMut<'a, N>) -> Self {
        Entry::Occupied(value.into())
    }
}

impl<'a, N> From<Node<'a, N>> for Entry<'a, N> {
    fn from(value: Node<'a, N>) -> Self {
        Entry::Occupied(value)
    }
}

/// For an entry where node does not exist.
#[derive(Debug)]
pub struct VacantEntry<'a, N>
where
    N: 'a,
{
    pub(crate) tree: &'a EytzingerTree<N>,
    pub(crate) index: usize,
}

impl<'a, N> Copy for VacantEntry<'a, N> {}

impl<'a, N> Clone for VacantEntry<'a, N> {
    fn clone(&self) -> Self {
        VacantEntry {
            tree: self.tree,
            index: self.index,
        }
    }
}

impl<'a, 'b, N> From<&'b VacantEntryMut<'a, N>> for VacantEntry<'b, N> {
    fn from(value: &'b VacantEntryMut<'a, N>) -> Self {
        VacantEntry {
            tree: value.tree,
            index: value.index,
        }
    }
}

impl<'a, N> From<VacantEntryMut<'a, N>> for VacantEntry<'a, N> {
    fn from(value: VacantEntryMut<'a, N>) -> Self {
        VacantEntry {
            tree: value.tree,
            index: value.index,
        }
    }
}

impl<'a, N> VacantEntry<'a, N> {
    /// Gets the Eytzinger tree this entry is for.
    pub fn tree(&self) -> &EytzingerTree<N> {
        self.tree
    }

    /// Gets the parent of this entry or `None` is there was none (i.e. if this entry is for the root).
    pub fn parent(&self) -> Option<Node<'a, N>> {
        self.tree.parent(self.index)
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
    pub fn parent(&self) -> Option<Node<'a, N>> {
        match self {
            Entry::Occupied(node) => node.parent(),
            Entry::Vacant(vacant_entry) => vacant_entry.parent(),
        }
    }

    /// Gets the node this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The node if there was one, `None` otherwise.
    pub fn node(&self) -> Option<Node<'a, N>> {
        match self {
            Entry::Occupied(node) => Some(*node),
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

    pub fn is_occupied(&self) -> bool {
        matches!(self, Entry::Occupied(_))
    }

    pub fn is_vacant(&self) -> bool {
        matches!(self, Entry::Vacant(_))
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
