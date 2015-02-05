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

    /// Create an empty treap.
    ///
    /// # Example
    ///
    /// ```
    /// use treap::Treap;
    /// let mut t = Treap::new();
    /// t.insert(5, "yellow");
    /// ```
    pub fn new() -> Treap<K, V> {
        Treap { root: None }
    }

    /// Borrow the value corresponding to the given key if it exists in the treap.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = treap::Treap::new();
    /// t.insert(5, "yellow");
    /// assert_eq!(t.get(&5), Some(&"yellow"));
    /// assert_eq!(t.get(&10), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.root {
            None => None,
            Some(ref n) => n.get(key)
        }
    }

    /// Insert a value with a given key. Returns the previous value if the key is already in the
    /// treap.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = treap::Treap::new();
    /// assert_eq!(t.insert(5, "yellow"), None);
    /// assert_eq!(t.insert(5, "blue"), Some("yellow"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        Node::insert_or_replace(&mut self.root, Node::new(key, value))
    }

    /// Return an iterator over keys and values in the treap. The order is arbitrary.
    ///
    /// ```
    /// let mut t = treap::Treap::new();
    /// t.insert(1, "red");
    /// t.insert(2, "blue");
    /// t.insert(3, "green");
    ///
    /// // Print keys and values in arbitrary order.
    /// for (k, v) in t.iter() {
    ///     println!("{}: {}", k, v);
    /// }
    /// ```
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
