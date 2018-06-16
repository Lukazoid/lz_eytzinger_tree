use std::ops::Range;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct EytzingerIndexCalculator {
    max_children_per_node: usize,
}

impl EytzingerIndexCalculator {
    pub fn new(max_children_per_node: usize) -> Self {
        assert!(max_children_per_node > 0);

        Self {
            max_children_per_node,
        }
    }

    pub fn max_children_per_node(&self) -> usize {
        self.max_children_per_node
    }

    pub fn child_index(&self, parent_index: usize, child_offset: usize) -> usize {
        assert!(
            child_offset < self.max_children_per_node,
            "the child index should be less than max_children_per_node"
        );

        (parent_index * self.max_children_per_node) + child_offset + 1
    }

    pub fn parent_index(&self, child_index: usize) -> Option<usize> {
        if child_index == 0 {
            None
        } else {
            Some((child_index - 1) / self.max_children_per_node)
        }
    }

    pub fn child_indexes(&self, parent_index: usize) -> Range<usize> {
        let first_child_index = self.child_index(parent_index, 0);

        first_child_index..(first_child_index + self.max_children_per_node)
    }
}
