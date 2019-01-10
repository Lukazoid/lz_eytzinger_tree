use crate::{DepthFirstOrder, EytzingerTree};

/// A depth-first iterator which returns owned values.
#[derive(Debug, Clone)]
pub struct DepthFirstIterator<N> {
    order: DepthFirstOrder,
    tree: EytzingerTree<N>,
    index: usize,
}

impl<N> DepthFirstIterator<N> {
    pub(crate) fn new(tree: EytzingerTree<N>, order: DepthFirstOrder) -> Self {
        Self {
            order,
            tree,
            index: 0,
        }
    }

    /// Gets the order of depth-first iteration.
    pub fn order(&self) -> DepthFirstOrder {
        self.order
    }
}

impl<N> Iterator for DepthFirstIterator<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.tree
                .value(self.index)
                .and_then(|v| v.as_ref())
                .is_some()
            {
                let current_index = self.index;
                self.index = self.tree.child_index(current_index, 0);
                if matches!(self.order, DepthFirstOrder::PreOrder) {
                    let value = self.tree
                        .value_mut(current_index)
                        .and_then(|v| v.take())
                        .expect("the value should not have been taken already");
                    return Some(value);
                }
            } else {
                if let Some(parent_index) = self.tree.parent_index(self.index) {
                    let node_child_offset = self.index - self.tree.child_index(parent_index, 0);
                    let next_child_offset = node_child_offset + 1;
                    if next_child_offset < self.tree.max_children_per_node() {
                        // try the next sibling
                        self.index = self.tree.child_index(parent_index, next_child_offset);
                    } else {
                        self.index = parent_index;

                        let removed_value = self.tree.remove(parent_index);
                        if matches!(self.order, DepthFirstOrder::PostOrder) {
                            return Some(
                                removed_value
                                    .expect("the value should not have been taken already"),
                            );
                        }
                    }
                } else {
                    // we have returned back to the root
                    return None;
                }
            }
        }
    }
}
