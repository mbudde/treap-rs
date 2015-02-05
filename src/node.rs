use std::mem;
use std::rand;
use std::cmp::Ordering;
use std::ops::{Deref,DerefMut};

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    priority: f64,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
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
                self.left.as_ref().and_then(|n| n.deref().get(key))
            }
            Ordering::Greater => {
                self.right.as_ref().and_then(|n| n.deref().get(key))
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
                boxed_node.deref_mut().insert(new)
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

    #[inline]
    fn is_heap_property_violated(&self, subtree: &Option<Box<Node<K, V>>>) -> bool {
        match *subtree {
            None => false,
            Some(ref b) => self.priority < b.deref().priority
        }
    }

    fn right_rotate(&mut self) {
        let left = mem::replace(&mut self.left, None);
        if let Some(mut boxed) = left {
            mem::swap(self, boxed.deref_mut());
            mem::swap(&mut self.right, &mut boxed.deref_mut().left);
            mem::replace(&mut self.right, Some(boxed));
        }
    }

    fn left_rotate(&mut self) {
        let right = mem::replace(&mut self.right, None);
        if let Some(mut boxed) = right {
            mem::swap(self, boxed.deref_mut());
            mem::swap(&mut self.left, &mut boxed.deref_mut().right);
            mem::replace(&mut self.left, Some(boxed));
        }
    }
}

