pub struct BTree {
    root: Option<Box<Node>>,
}

struct Node {
    keys: Vec<u64>,
    children: Vec<Option<Box<Node>>>,
    is_leaf: bool,
}

impl BTree {
    pub fn new() -> Self {
        BTree { root: None }
    }

    pub fn insert(&mut self, key: u64, _value: Vec<u8>) {
        // Implement the insert method
        // For simplicity, this example does not store the value
        if let Some(mut root) = self.root.take() {
            // Insert into existing tree
            self.insert_non_full(&mut root, key);
            self.root = Some(root);
        } else {
            // Create a new root node
            let root = Node {
                keys: vec![key],
                children: vec![],
                is_leaf: true,
            };
            self.root = Some(Box::new(root));
        }
    }

    fn insert_non_full(&mut self, node: &mut Box<Node>, key: u64) {
        // Insert key into a non-full node
        if node.is_leaf {
            node.keys.push(key);
            node.keys.sort();
        } else {
            // Find the child to insert into
            let mut i = 0;
            while i < node.keys.len() && key > node.keys[i] {
                i += 1;
            }
            if let Some(child) = &mut node.children[i] {
                self.insert_non_full(child, key);
            }
        }
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, _key: u64) {
        // Implement the delete method
        // For simplicity, this example does not implement deletion
    }

    pub fn search(&self, key: u64) -> bool {
        // Implement the search method
        self.search_node(&self.root, key)
    }

    fn search_node(&self, node: &Option<Box<Node>>, key: u64) -> bool {
        if let Some(n) = node {
            if n.keys.contains(&key) {
                return true;
            }
            for child in &n.children {
                if self.search_node(child, key) {
                    return true;
                }
            }
        }
        false
    }
}