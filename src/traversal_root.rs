use {EtzyngerTree, Node};

#[derive(Debug)]
pub(crate) enum TraversalRoot<'a, N>
where
    N: 'a,
{
    Tree(&'a EtzyngerTree<N>),
    Node(Node<'a, N>),
}

impl<'a, N> TraversalRoot<'a, N> {
    pub fn starting_node(&self) -> Option<Node<'a, N>> {
        match self {
            TraversalRoot::Node(node) => Some(*node),
            _ => None,
        }
    }

    pub fn tree(&self) -> &'a EtzyngerTree<N> {
        match self {
            TraversalRoot::Tree(tree) => tree,
            TraversalRoot::Node(node) => node.tree(),
        }
    }
}

impl<'a, N> Clone for TraversalRoot<'a, N> {
    fn clone(&self) -> Self {
        match self {
            TraversalRoot::Tree(tree) => TraversalRoot::Tree(tree),
            TraversalRoot::Node(node) => TraversalRoot::Node(*node),
        }
    }
}

impl<'a, N> Copy for TraversalRoot<'a, N> {}
