


#[derive(Debug, Clone)]
pub enum NodeType {
    Leaf,
    Internal,
}

pub type NodeIndex = usize;

#[derive(Debug, Clone)]
pub struct Node {
    keys: Vec<i32>,
    children: Option<Vec<NodeIndex>>,
    node_type: NodeType,
    order: usize,
}

impl Node {
    fn new_leaf(order: usize) -> Self {
        Node {
            keys: Vec::with_capacity(order - 1),
            children: None,
            node_type: NodeType::Leaf,
            order,
        }
    }

    fn new_internal(order: usize) -> Self {
        Node {
            keys: Vec::with_capacity(order - 1),
            children: Some(Vec::with_capacity(order)),
            node_type: NodeType::Internal,
            order,
        }
    }

    fn verify_node(&self) {
        match self.node_type {
            NodeType::Leaf => {
                assert!(
                    self.children.is_none(),
                    "Leaf node should not have children"
                );
                assert!(
                    self.keys.len() <= self.order - 1,
                    "Leaf node should have less than {} keys",
                    self.order
                );
            }
            NodeType::Internal => {
                assert!(
                    self.children.is_some(),
                    "Internal node should have children"
                );
                assert!(
                    self.keys.len() <= self.order - 1,
                    "Internal node should have less than {} keys",
                    self.order
                );
                assert!(
                    self.keys.len() + 1 == self.children.as_ref().expect("failed to get child").len(),
                    "Internal node should have one more child than keys: {:?}, children: {:?}",
                    self.keys,
                    self.children.as_ref().expect("failed to get child")
                );
            }
        }
    }

}

#[derive(Debug)]
pub struct Btree {
    pub root: Option<NodeIndex>,
    order: usize,
    pub nodes: Vec<Node>, // it probably also needs a free list to use that or maybe find an allocatior online
}

impl Btree {
    pub fn new(order: usize) -> Self {
        Btree {
            root: None,
            order,
            nodes: Vec::with_capacity(1000),
        }
    }

    pub fn search(&self, key: i32) -> bool {
        match self.root {
            Some(index) => self.search_inner(index, key),
            None => false,
        }
    }

    fn search_inner(&self, node_index: NodeIndex, key: i32) -> bool {
        match self.nodes[node_index].node_type {
            NodeType::Leaf => {
                let node = &self.nodes[node_index];
                node.keys.binary_search(&key).is_ok()
            },
            NodeType::Internal => {
                let node = &self.nodes[node_index];
                let child_index = node.keys.partition_point(|&k| k < key);
                self.search_inner(node.children.as_ref().expect("failed to get child")[child_index], key)
            }
        }
    }

    pub fn insert(&mut self, key: i32) {
        if key == 15 {
            println!("inserting 15");
        }
        match self.root {
            Some(index) => {
                let root = self.insert_inner(index, key);
                if let Some(index) = root {
                    self.root = Some(index);
                    self.nodes[index].verify_node();
                }
            },
            None => {
                self.nodes.push(Node::new_leaf(self.order));
                let root = self.nodes.len() - 1;
                self.nodes[root].keys.push(key);
                self.nodes[root].verify_node();
                self.root = Some(root);
            },
        }
    }

    fn insert_inner(&mut self, node_index: NodeIndex, key: i32) -> Option<NodeIndex> {
        let mut node = self.nodes[node_index].clone();
        let insertion_index = node.keys.partition_point(|&k| k <= key);

        match node.node_type {
            NodeType::Leaf => {
                let keys = &mut node.keys;
                keys.insert(insertion_index, key);

                // split the current node if it is full
                if node.keys.len() == self.order {
                    let (left, parent, right) = split_node(self.order, node.clone());

                    self.nodes.push(left);
                    let left_index = self.nodes.len() - 1;
                    
                    self.nodes.push(right);
                    let right_index = self.nodes.len() - 1;

                    self.nodes.push(parent);
                    let parent_index = self.nodes.len() - 1;

                    self.nodes[parent_index].children.as_mut().expect("failed to get child").push(left_index);
                    self.nodes[parent_index].children.as_mut().expect("failed to get child").push(right_index);
                    self.nodes[parent_index].verify_node();
                    return Some(parent_index);
                }
                node.verify_node();
            },
            NodeType::Internal => {
                let child = node.children.as_ref().expect("failed to get child")[insertion_index];
                let inserted_node_index = self.insert_inner(child, key);

                if let Some(index) = inserted_node_index {
                    let split_node = self.nodes[index].clone();
                    let left_child = split_node.children.as_ref().expect("failed to get child")[0];
                    let right_child = split_node.children.as_ref().expect("failed to get child")[1];
                    
                    node.keys.insert(insertion_index, split_node.keys[0]);
                    node.children.as_mut().expect("failed to get child")[insertion_index] = left_child;
                    node.children.as_mut().expect("failed to get child").insert(insertion_index + 1, right_child);
                }

                // split the current node if it is full
                if node.keys.len() == self.order {
                    let (left, parent, right) = split_node(self.order, node.clone());

                    self.nodes.push(left);
                    let left_index = self.nodes.len() - 1;

                    self.nodes.push(right);
                    let right_index = self.nodes.len() - 1;

                    self.nodes.push(parent);
                    let parent_index = self.nodes.len() - 1;
                    self.nodes[parent_index].children.as_mut().expect("failed to get child").push(left_index);
                    self.nodes[parent_index].children.as_mut().expect("failed to get child").push(right_index);
                    self.nodes[parent_index].verify_node();

                    return Some(parent_index);
                }
                node.verify_node();
            }
        }

        self.nodes[node_index] = node;
        return None;
    }
}

fn split_node(order: usize, node: Node) -> (Node, Node, Node) {
    let mid = node.keys.len() / 2;
    match node.node_type {
        NodeType::Leaf => {
            let mut left = Node::new_leaf(order);
            left.keys.extend_from_slice(&node.keys[..mid + 1]);

            let mut right = Node::new_leaf(order);
            right.keys.extend_from_slice(&node.keys[mid + 1..]);

            let mut parent = Node::new_internal(order);
            parent.keys.push(node.keys[mid]);

            return (left, parent, right)
        }
        NodeType::Internal => {
            let mut left = Node::new_internal(order);
            left.keys.extend_from_slice(&node.keys[..mid + 1]);
            left.children.as_mut().expect("failed to get child").extend_from_slice(&node.children.as_ref().expect("failed to get child")[..mid + 1]);

            let mut right = Node::new_internal(order);
            right.keys.extend_from_slice(&node.keys[mid + 1..]);
            right.children.as_mut().expect("failed to get child").extend_from_slice(&node.children.as_ref().expect("failed to get child")[mid + 1..]);

            let mut parent = Node::new_internal(order);
            parent.keys.push(node.keys[mid]);

            return (left, parent, right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_insertion() {
        let mut btree = Btree::new(3);
        for i in 0..10000 {
            btree.insert(i);
        }
    }
}
