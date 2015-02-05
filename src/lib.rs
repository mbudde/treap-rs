use std::ops::{Deref};

mod node;

use node::{Node};

#[derive(Debug)]
pub struct Treap<K, V> {
    root: Option<Box<Node<K, V>>>,
}

pub struct Iter<'a, K: 'a, V: 'a> {
    nodes: Vec<&'a Node<K, V>>,
}

impl<K: Ord, V> Treap<K, V> {
    pub fn new() -> Treap<K, V> {
        Treap { root: None }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.root {
            None => None,
            Some(ref n) => n.get(key)
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        Node::insert_or_replace(&mut self.root, Node::new(key, value))
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
