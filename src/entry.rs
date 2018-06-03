use {EytzingerTree, NodeMut};

/// An entry can be used to reference a node in an eytzinger tree. The node may or may not have a
/// value.
#[derive(Debug)]
pub enum Entry<'a, N>
where
    N: 'a,
{
    /// When the entry references a node which exists with a value.
    Occupied(OccupiedEntry<'a, N>),

    /// When the entry references a non-existent node.
    Vacant(VacantEntry<'a, N>),
}

/// For an entry where the node exists with a value.
#[derive(Debug)]
pub struct OccupiedEntry<'a, N>
where
    N: 'a,
{
    pub(crate) node: NodeMut<'a, N>,
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

impl<'a, N> Entry<'a, N> {
    /// Inserts a value at the referenced position of there is no node already there.
    pub fn or_insert(self, value: N) -> NodeMut<'a, N> {
        match self {
            Entry::Occupied(occupied) => occupied.node,
            Entry::Vacant(vacant) => vacant.tree.set_value(vacant.index, Some(value)),
        }
    }

    /// Inserts a value at the referenced position of there is no node already there.
    pub fn or_insert_with<F>(self, value_factory: F) -> NodeMut<'a, N>
    where
        F: FnOnce() -> N,
    {
        match self {
            Entry::Occupied(occupied) => occupied.node,
            Entry::Vacant(vacant) => vacant.tree.set_value(vacant.index, Some(value_factory())),
        }
    }

    /// Modifies the value (if one exists).
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut N),
    {
        match self {
            Entry::Occupied(mut occupied) => {
                f(occupied.node.value_mut());
                Entry::Occupied(occupied)
            }
            entry @ Entry::Vacant(_) => entry,
        }
    }

    /// Removes the node if one existed.
    pub fn remove(self) {
        match self {
            Entry::Occupied(occupied) => occupied.node.remove(),
            Entry::Vacant(_) => {}
        }
    }
}
