
use std::rand;
use std::fmt;
use std::mem;
use std::cmp::Ordering;
use std::ops::{Deref,DerefMut};

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    value: V,
    priority: f64,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
pub struct Treap<K, V> {
    root: Option<Box<Node<K, V>>>,
}

pub struct Iter<'a, K: 'a, V: 'a> {
    nodes: Vec<&'a Node<K, V>>,
}

impl<K: Ord, V> Node<K, V> {

    fn insert_or_replace(subtree: &mut Option<Box<Node<K, V>>>, new: Node<K, V>) {
        match *subtree {
            None => {
                mem::replace(subtree, Some(Box::new(new)));
            }
            Some(ref mut boxed_node) => {
                boxed_node.deref_mut().insert(new);
            }
        }
    }

    fn insert(&mut self, node: Node<K, V>) {
        match node.key.cmp(&self.key) {
            Ordering::Equal   => self.value = node.value,
            Ordering::Less    => {
                Node::insert_or_replace(&mut self.left, node);
                if self.is_heap_property_violated(&self.left) {
                    self.right_rotate();
                }
            }
            Ordering::Greater => {
                Node::insert_or_replace(&mut self.right, node);
                if self.is_heap_property_violated(&self.right) {
                    self.left_rotate();
                }
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


impl<K: Ord, V> Treap<K, V> {
    pub fn new() -> Treap<K, V> {
        Treap { root: None }
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        let mut node = &self.root;
        loop {
            match *node {
                None        => return None,
                Some(ref n) => {
                    match key.cmp(&n.key) {
                        Ordering::Equal   => return Some(&n.value),
                        Ordering::Less    => node = &n.left,
                        Ordering::Greater => node = &n.right,
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let new_node = Node {
            key: key,
            value: value,
            priority: rand::random(),
            left: None,
            right: None,
        };
        Node::insert_or_replace(&mut self.root, new_node);
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter {
            nodes: match self.root {
                None => Vec::new(),
                Some(ref n) => vec![n.deref()]
            }
        }
    }

}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        match self.nodes.pop() {
            None => None,
            Some(node) => {
                if let Some(ref boxed) = node.left {
                    self.nodes.push(boxed.deref());
                }
                if let Some(ref boxed) = node.right {
                    self.nodes.push(boxed.deref());
                }
                Some((&node.key, &node.value))
            }
        }
    }
}
