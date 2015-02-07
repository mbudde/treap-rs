use std::mem;
use std::rand;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    priority: f64,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
}

enum DeleteCases {
    DeleteNode,
    RotateLeft,
    RotateRight
}

impl<K: Ord, V> Node<K, V> {

    pub fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key: key,
            value: value,
            priority: rand::random(),
            left: None,
            right: None,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.key.cmp(key) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => {
                self.left.as_ref().and_then(|n| n.get(key))
            }
            Ordering::Greater => {
                self.right.as_ref().and_then(|n| n.get(key))
            }
        }
    }

    pub fn insert_or_replace(subtree: &mut Option<Box<Node<K, V>>>, new: Node<K, V>) -> Option<V> {
        match *subtree {
            None => {
                mem::replace(subtree, Some(Box::new(new)));
                None
            }
            Some(ref mut boxed_node) => {
                boxed_node.insert(new)
            }
        }
    }

    pub fn insert(&mut self, node: Node<K, V>) -> Option<V> {
        match node.key.cmp(&self.key) {
            Ordering::Equal => { Some(mem::replace(&mut self.value, node.value)) }
            Ordering::Less => {
                let old_value = Node::insert_or_replace(&mut self.left, node);
                if self.is_heap_property_violated(&self.left) {
                    self.right_rotate();
                }
                old_value
            }
            Ordering::Greater => {
                let old_value = Node::insert_or_replace(&mut self.right, node);
                if self.is_heap_property_violated(&self.right) {
                    self.left_rotate();
                }
                old_value
            }
        }
    }

    pub fn delete(subtree: &mut Option<Box<Node<K, V>>>, key: &K) -> Option<V> {
        {
            let node = match *subtree {
                None => return None,
                Some(ref mut n) => n
            };
            match key.cmp(&node.key) {
                Ordering::Less    => { return Node::delete(&mut node.left, key) }
                Ordering::Greater => { return Node::delete(&mut node.right, key) }
                Ordering::Equal => {}
            }
        }
        Node::rotate_down(subtree)
    }

    fn rotate_down(subtree: &mut Option<Box<Node<K, V>>>) -> Option<V> {
        let case = match *subtree {
            None => return None,
            Some(ref root) => {
                match (&root.left, &root.right) {
                    (&None, &None) => { DeleteCases::DeleteNode }
                    (&Some(ref left), &Some(ref right)) => {
                        if left.priority >= right.priority {
                            DeleteCases::RotateRight
                        } else {
                            DeleteCases::RotateLeft
                        }
                    }
                    (&Some(_), &None) => { DeleteCases::RotateRight }
                    (&None, &Some(_)) => { DeleteCases::RotateLeft }
                }
            }
        };
        match case {
            DeleteCases::DeleteNode => {
                subtree.take().map(|n| n.value)
            }
            DeleteCases::RotateLeft => {
                subtree.as_mut().and_then(|n| { n.left_rotate(); Node::rotate_down(&mut n.left) })
            }
            DeleteCases::RotateRight => {
                subtree.as_mut().and_then(|n| { n.right_rotate(); Node::rotate_down(&mut n.right) })
            }
        }
    }

    #[inline]
    fn is_heap_property_violated(&self, subtree: &Option<Box<Node<K, V>>>) -> bool {
        match *subtree {
            None => false,
            Some(ref b) => self.priority < b.priority
        }
    }

    fn right_rotate(&mut self) {
        let left = mem::replace(&mut self.left, None);
        if let Some(mut boxed) = left {
            mem::swap(self, &mut *boxed);
            mem::swap(&mut self.right, &mut boxed.left);
            mem::replace(&mut self.right, Some(boxed));
        }
    }

    fn left_rotate(&mut self) {
        let right = mem::replace(&mut self.right, None);
        if let Some(mut boxed) = right {
            mem::swap(self, &mut *boxed);
            mem::swap(&mut self.left, &mut boxed.right);
            mem::replace(&mut self.left, Some(boxed));
        }
    }
}

