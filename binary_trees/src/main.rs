use std::fmt;
#[derive(Debug)]
struct BinaryTree<T> {
    nodes: Vec<Option<T>>,
}

impl<T> BinaryTree<T>
where
    T: Copy + fmt::Debug,
{
    fn new() -> Self {
        BinaryTree { nodes: Vec::new() }
    }
    fn insert(&mut self, value: T) {
        self.nodes.push(Some(value));
    }
    fn get_left_child(&self, index: usize) -> Option<T> {
        let left_index = 2 * index + 1;
        if left_index < self.nodes.len() {
            self.nodes[left_index]
        } else {
            None
        }
    }
    fn get_right_child(&self, index: usize) -> Option<T> {
        let right_index = 2 * index + 2;
        if right_index < self.nodes.len() {
            self.nodes[right_index]
        } else {
            return None;
        }
    }
    fn get_parent(&self, index: usize) -> Option<T> {
        if index == 0 {
            return None;
        }
        let parent_index = (index - 1) / 2;
        if parent_index < self.nodes.len() {
            self.nodes[parent_index]
        } else {
            return None;
        }
    }
}
fn main() {
    let mut tree = BinaryTree::new();
    tree.insert('a');
    tree.insert('b');
    tree.insert('c');
    tree.insert('d');
    tree.insert('e');
    tree.insert('f');
    tree.insert('g');

    println!("The left child of A is {:?}", tree.get_left_child(0));
    println!("The left child of A is {:?}", tree.get_right_child(0));
    println!("The left child of A is {:?}", tree.get_parent(0))
}
