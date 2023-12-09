#[derive(Debug, Clone)]
struct Node {
    value: i32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left_child: None,
            right_child: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value > self.value {
            if let Some(ref mut right_child) = self.right_child {
                right_child.insert(value);
            } else {
                self.right_child = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(ref mut left_child) = self.left_child {
                left_child.insert(value);
            } else {
                self.left_child = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn search(&self, value: i32) -> Option<&Node> {
        if self.value == value {
            return Some(self);
        }

        if value < self.value {
            if let Some(ref left) = self.left_child {
                return left.search(value);
            }
        } else {
            if let Some(ref right) = self.right_child {
                return right.search(value);
            }
        }

        None
    }
    //This code generate GPT and is not finished
    pub fn delete(&mut self, value: i32) -> Option<Box<Node>> {
        if value < self.value {
            if let Some(ref mut left) = self.left_child.take() {
                self.left_child = left.delete(value);
            }
        } else if value > self.value {
            if let Some(ref mut right) = self.right_child.take() {
                self.right_child = right.delete(value);
            }
        } else {
            if self.left_child.is_none() {
                return self.right_child.take();
            } else if self.right_child.is_none() {
                return self.left_child.take();
            }
            let mut right_min = self.right_child.as_mut().unwrap();
            let mut tmp = right_min.delete_min();
            std::mem::swap(&mut right_min.value, &mut self.value);
            self.right_child = tmp;
        }
        None
    }

    fn delete_min(&mut self) -> Option<Box<Node>> {
        if self.left_child.is_none() {
            return self.right_child.take();
        }
        let left_min = self.left_child.as_mut().unwrap();
        let tmp = left_min.delete_min();
        std::mem::swap(&mut left_min.value, &mut self.value);
        self.left_child = tmp;
        Some(Box::new(self.clone()))
    }
}

fn main() {
    let mut node = Node::new(42);

    let node2 = 37;
    let node3 = 100;
    let node4 = 20;
    let node5 = 38;
    let node6 = 98;
    let node7 = 156;
    let node8 = 101;

    node.insert(node2);
    node.insert(node3);
    node.insert(node4);
    node.insert(node5);
    node.insert(node6);
    node.insert(node7);
    node.insert(node8);

    println!("{:#?}", node);

    node.delete(20);

    println!("{:#?}", node);
}
