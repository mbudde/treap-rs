
use std::iter::{FromIterator};

use node::{Node};

/// A map based on a randomized treap
#[derive(Debug, Clone)]
pub struct TreapMap<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> TreapMap<K, V> {

    /// Create an empty treap.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, "yellow");
    /// if let Some(s) = t.get(&5) {
    ///     println!("{}", s);
    /// }
    /// ```
    pub fn new() -> TreapMap<K, V> {
        TreapMap { root: None }
    }

    /// Borrow the value corresponding to the given key if it exists in the treap.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, "yellow");
    /// t.insert(3, "blue");
    /// t.insert(8, "green");
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
    /// let mut t = treap::TreapMap::new();
    /// assert_eq!(t.insert(5, "yellow"), None);
    /// assert_eq!(t.insert(5, "blue"), Some("yellow"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        Node::insert_or_replace(&mut self.root, Node::new(key, value))
    }

    /// Delete the given key from the treap and return the value associated with it if any.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, "blue");
    /// assert_eq!(t.delete(&5), Some("blue"));
    /// assert_eq!(t.delete(&10), None);
    /// ```
    pub fn delete(&mut self, key: &K) -> Option<V> {
        Node::delete(&mut self.root, key)
    }

    /// Return an iterator over keys and values in the treap. The order is arbitrary.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.extend(vec![(1, 200), (2, 120), (3, 330)].into_iter());
    ///
    /// let sum = t.iter().fold(0, |s, (&k, &v)| s + k + v);
    /// assert_eq!(sum, 656);
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, K, V> {
        Iter {
            nodes: match self.root {
                None => Vec::new(),
                Some(ref n) => vec![&**n]
            }
        }
    }

    /// Return an iterator that moves keys and values out of treap. The order is arbitrary.
    ///
    /// # Example
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.extend(vec![(1, "red"), (2, "blue"), (3, "green")].into_iter());
    ///
    /// // Print keys and values in arbitrary order.
    /// for (k, v) in t.into_iter() {
    ///     println!("{}: {}", k, v);
    /// }
    /// ```
    pub fn into_iter(self) -> IntoIter<K, V> {
        IntoIter {
            nodes: match self.root {
                None => Vec::new(),
                Some(n) => vec![*n]
            }
        }
    }
}

impl<K: Ord, V> Extend<(K, V)> for TreapMap<K, V> {
    #[inline]
    fn extend<T: Iterator<Item=(K, V)>>(&mut self, mut iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for TreapMap<K, V> {
    #[inline]
    fn from_iter<T: Iterator<Item=(K, V)>>(iter: T) -> TreapMap<K, V> {
        let mut treap = TreapMap::new();
        treap.extend(iter);
        treap
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    nodes: Vec<&'a Node<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        match self.nodes.pop() {
            None => None,
            Some(node) => {
                if let Some(ref boxed) = node.left {
                    self.nodes.push(&**boxed);
                }
                if let Some(ref boxed) = node.right {
                    self.nodes.push(&**boxed);
                }
                Some((&node.key, &node.value))
            }
        }
    }
}

pub struct IntoIter<K, V> {
    nodes: Vec<Node<K, V>>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        match self.nodes.pop() {
            None => None,
            Some(node) => {
                if let Some(boxed) = node.left {
                    self.nodes.push(*boxed);
                }
                if let Some(boxed) = node.right {
                    self.nodes.push(*boxed);
                }
                Some((node.key, node.value))
            }
        }
    }
}
