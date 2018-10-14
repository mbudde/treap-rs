use std::cmp::Ordering;
use std::mem;

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    priority: f64, // TODO: use a u64! much faster!
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
}

enum RemovalCases {
    RemoveNode,
    RotateLeft,
    RotateRight,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V, priority: f64) -> Node<K, V> {
        Node {
            key,
            value,
            priority,
            left: None,
            right: None,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.left.as_ref().and_then(|n| n.get(key)),
            Ordering::Greater => self.right.as_ref().and_then(|n| n.get(key)),
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match key.cmp(&self.key) {
            Ordering::Equal => Some(&mut self.value),
            Ordering::Less => self.left.as_mut().and_then(|n| n.get_mut(key)),
            Ordering::Greater => self.right.as_mut().and_then(|n| n.get_mut(key)),
        }
    }

    pub fn insert_or_replace(subtree: &mut Option<Box<Node<K, V>>>, new: Node<K, V>) -> Option<V> {
        match *subtree {
            None => {
                mem::replace(subtree, Some(Box::new(new)));
                None
            }
            Some(ref mut node) => node.insert(new),
        }
    }

    pub fn insert(&mut self, node: Node<K, V>) -> Option<V> {
        match node.key.cmp(&self.key) {
            Ordering::Equal => {
                if self.priority < node.priority {
                    self.priority = node.priority;
                }
                Some(mem::replace(&mut self.value, node.value))
            }
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

    pub fn remove(subtree: &mut Option<Box<Node<K, V>>>, key: &K) -> Option<V> {
        {
            let node = match *subtree {
                None => return None,
                Some(ref mut n) => n,
            };
            match key.cmp(&node.key) {
                Ordering::Less => return Node::remove(&mut node.left, key),
                Ordering::Greater => return Node::remove(&mut node.right, key),
                Ordering::Equal => {}
            }
        }
        Node::rotate_down(subtree)
    }

    fn rotate_down(subtree: &mut Option<Box<Node<K, V>>>) -> Option<V> {
        let case = match *subtree {
            None => return None,
            Some(ref root) => match (&root.left, &root.right) {
                (&None, &None) => RemovalCases::RemoveNode,
                (&Some(ref left), &Some(ref right)) => {
                    if left.priority >= right.priority {
                        RemovalCases::RotateRight
                    } else {
                        RemovalCases::RotateLeft
                    }
                }
                (&Some(_), &None) => RemovalCases::RotateRight,
                (&None, &Some(_)) => RemovalCases::RotateLeft,
            },
        };
        match case {
            RemovalCases::RemoveNode => subtree.take().map(|n| n.value),
            RemovalCases::RotateLeft => subtree.as_mut().and_then(|n| {
                n.left_rotate();
                Node::rotate_down(&mut n.left)
            }),
            RemovalCases::RotateRight => subtree.as_mut().and_then(|n| {
                n.right_rotate();
                Node::rotate_down(&mut n.right)
            }),
        }
    }

    #[inline]
    fn is_heap_property_violated(&self, subtree: &Option<Box<Node<K, V>>>) -> bool {
        match *subtree {
            None => false,
            Some(ref b) => self.priority < b.priority,
        }
    }

    //       q               p
    //      / \             / \
    //     p  C   --->     A  q
    //    / \                / \
    //   A  B               B  C
    fn right_rotate(&mut self) {
        // Cut left subtree of q
        let left = mem::replace(&mut self.left, None);
        if let Some(mut node) = left {
            // Let subtree p be root and `node` point to q
            mem::swap(self, &mut *node);
            // Move subtree B from p to left subtree of q
            mem::swap(&mut self.right, &mut node.left);
            // Let q be right child of p
            mem::replace(&mut self.right, Some(node));
        }
    }

    //     p               q
    //    / \             / \
    //   A  q   --->     p  C
    //     / \          / \
    //    B  C         A  B
    fn left_rotate(&mut self) {
        // Cut right subtree of p
        let right = mem::replace(&mut self.right, None);
        if let Some(mut node) = right {
            // Let subtree q be root and `node` point to p
            mem::swap(self, &mut *node);
            // Move subtree B from q to right subtree of p
            mem::swap(&mut self.left, &mut node.right);
            // Let p be left child of q
            mem::replace(&mut self.left, Some(node));
        }
    }
}
