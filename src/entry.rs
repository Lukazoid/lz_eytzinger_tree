use {EytzingerTree, NodeMut};

#[derive(Debug)]
pub enum Entry<'a, N>
where
    N: 'a,
{
    Occupied(OccupiedEntry<'a, N>),
    Vacant(VacantEntry<'a, N>),
}

#[derive(Debug)]
pub struct OccupiedEntry<'a, N>
where
    N: 'a,
{
    pub(crate) node: NodeMut<'a, N>,
}

#[derive(Debug)]
pub struct VacantEntry<'a, N>
where
    N: 'a,
{
    pub(crate) tree: &'a mut EytzingerTree<N>,
    pub(crate) index: usize,
}

impl<'a, N> Entry<'a, N> {
    pub fn or_insert(self, value: N) -> NodeMut<'a, N> {
        match self {
            Entry::Occupied(occupied) => occupied.node,
            Entry::Vacant(vacant) => vacant.tree.set_value(vacant.index, Some(value)),
        }
    }

    pub fn or_insert_with<F>(self, value_factory: F) -> NodeMut<'a, N>
    where
        F: FnOnce() -> N,
    {
        match self {
            Entry::Occupied(occupied) => occupied.node,
            Entry::Vacant(vacant) => vacant.tree.set_value(vacant.index, Some(value_factory())),
        }
    }

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
}
