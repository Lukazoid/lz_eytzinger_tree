/// The order of depth-first iteration. This does NOT include in-order as the Etzyinger tree does
/// not guarantee the actual order of nodes by value.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DepthFirstOrder {
    /// Parent nodes are returned before their children.
    PreOrder,
    /// Child nodes are returned before their parents.
    PostOrder,
}
