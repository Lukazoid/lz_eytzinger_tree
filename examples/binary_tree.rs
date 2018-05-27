extern crate lz_etzynger_tree;

use lz_etzynger_tree::EtzyngerTree;

fn main() {
    let mut binary_tree = BinaryTree::new();
    binary_tree.insert(5);
    binary_tree.insert(2);
    binary_tree.insert(3);
    binary_tree.insert(1);
    binary_tree.insert(6);

    println!("tree: {:?}", binary_tree);
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    tree: EtzyngerTree<T>,
}

impl<T> BinaryTree<T> {
    fn new() -> Self {
        Self {
            tree: EtzyngerTree::new(2),
        }
    }
}

const LEFT: usize = 0;
const RIGHT: usize = 1;

impl<T: Ord> BinaryTree<T> {
    pub fn insert(&mut self, value: T) {
        if let Some(root) = self.tree.root_mut() {
            let mut current = root;
            loop {
                if &value == current.value() {
                    return;
                }

                if &value < current.value() {
                    match current.to_child(LEFT) {
                        Ok(left) => {
                            current = left;
                            continue;
                        }
                        Err(mut current) => {
                            current.set_child_value(LEFT, value);
                            return;
                        }
                    }
                }

                match current.to_child(RIGHT) {
                    Ok(right) => {
                        current = right;
                        continue;
                    }
                    Err(mut current) => {
                        current.set_child_value(RIGHT, value);
                        return;
                    }
                }
            }
        }
        self.tree.set_root_value(value);
    }
}
