use {EytzingerTree, NodeMut};

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
    /// The removed value if there was a node.
    pub fn remove(self) -> Option<N> {
        self.node().map(|n| n.remove())
    }

    /// Gets the mutable node this entry is for, if there is one.
    ///
    /// # Returns
    ///
    /// The mutable node if there was one, `None` otherwise.
    pub fn node(self) -> Option<NodeMut<'a, N>> {
        match self {
            Entry::Occupied(node) => Some(node),
            Entry::Vacant(_) => None,
        }
    }
}
