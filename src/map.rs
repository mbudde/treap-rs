
use std::default::Default;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Index, IndexMut};

use node::{Node};

/// A map based on a randomized treap.
#[derive(Debug, Clone)]
pub struct TreapMap<K, V> {
    root: Option<Box<Node<K, V>>>,
    size: usize,
}

/// An iterator over a treap's entries.
pub struct Iter<'a, K: 'a, V: 'a> {
    nodes: Vec<&'a Node<K, V>>,
}

/// A mutable iterator over a treap's entries.
pub struct IterMut<'a, K: 'a, V: 'a> {
    nodes: Vec<&'a mut Node<K, V>>,
}

/// An owning iterator over a treap's entries.
pub struct IntoIter<K, V> {
    nodes: Vec<Node<K, V>>,
}

enum Traversal<T> {
    // Traverse left subtree before emitting value at node
    Left(T),
    // Emit value at node and continue with right subtree
    Right(T),
}

/// An iterator over a treap's entries in key order.
pub struct OrderedIter<'a, K: 'a, V: 'a> {
    nodes: Vec<Traversal<&'a Node<K, V>>>,
}

impl<K: Ord, V> TreapMap<K, V> {

    /// Create an empty treap.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, "yellow");
    /// if let Some(s) = t.get(&5) {
    ///     println!("{}", s);
    /// }
    /// ```
    pub fn new() -> TreapMap<K, V> {
        TreapMap { root: None, size: 0 }
    }

    /// Return the number of elements in the treap.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// assert_eq!(t.len(), 0);
    /// t.insert(5, 1);
    /// assert_eq!(t.len(), 1);
    /// ```
    pub fn len(&self) -> usize { self.size }

    /// Return true if the treap contains no elements.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// assert!(t.is_empty());
    /// t.insert(5, 1);
    /// assert!(!t.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool { self.size == 0 }

    /// Removes all elements from the treap.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, 1);
    /// t.clear();
    /// assert!(t.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.root.take();
        self.size = 0;
    }

    /// Borrow the value corresponding to the given key if it exists in the treap.
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

    /// Return a mutable reference to the value corresponding to the given key if it exists in the treap.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, "yellow");
    /// match t.get_mut(&5) {
    ///     Some(x) => *x = "blue",
    ///     None => (),
    /// }
    /// assert_eq!(t.get(&5), Some(&"blue"));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.root {
            Some(ref mut n) => n.get_mut(key),
            None => None,
        }
    }

    /// Returns true if the key is present in the treap.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, "yellow");
    /// assert_eq!(t.contains_key(&5), true);
    /// assert_eq!(t.contains_key(&8), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Insert a value with a given key. Returns the previous value if the key is already in the
    /// treap.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// assert_eq!(t.insert(5, "yellow"), None);
    /// assert_eq!(t.insert(5, "blue"), Some("yellow"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let res = Node::insert_or_replace(&mut self.root, Node::new(key, value));
        if res.is_none() { self.size += 1; }
        res
    }

    /// Remove the given key from the treap and return the value associated with it if any.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.insert(5, "blue");
    /// assert_eq!(t.remove(&5), Some("blue"));
    /// assert_eq!(t.remove(&10), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let res = Node::remove(&mut self.root, key);
        if res.is_some() { self.size -= 1; }
        res
    }

    /// Returns an iterator over keys and values in the treap that gives the keys in sorted order.
    ///
    /// ```
    /// let mut t = treap::TreapMap::new();
    /// t.extend((1..10).map(|x| (x, "a")));
    ///
    /// let v: Vec<i32> = t.iter_ordered().map(|(&k, _)| k).collect();
    /// assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn iter_ordered(&self) -> OrderedIter<K, V> {
        OrderedIter {
            nodes: match self.root {
                None => Vec::new(),
                Some(ref n) => vec![Traversal::Left(&**n)]
            }
        }
    }
}

impl<K: Ord, V> Extend<(K, V)> for TreapMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item=(K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for TreapMap<K, V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item=(K, V)>>(iter: T) -> TreapMap<K, V> {
        let mut treap = TreapMap::new();
        treap.extend(iter);
        treap
    }
}

impl<K: Ord, V> Default for TreapMap<K, V> {
    fn default() -> TreapMap<K, V> {
        TreapMap::new()
    }
}

/// Return an iterator that moves keys and values out of treap. The order is arbitrary.
///
/// ```
/// let mut t = treap::TreapMap::new();
/// t.extend(vec![(1, "red"), (2, "blue"), (3, "green")].into_iter());
///
/// // Print keys and values in arbitrary order.
/// for (k, v) in t {
///     println!("{}: {}", k, v);
/// }
/// ```
impl<K: Ord, V> IntoIterator for TreapMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        IntoIter {
            nodes: match self.root {
                None => Vec::new(),
                Some(n) => vec![*n]
            }
        }
    }
}

/// Return an iterator over keys and values in the treap. The order is arbitrary.
///
/// ```
/// let mut t = treap::TreapMap::new();
/// t.extend(vec![(1, 200), (2, 120), (3, 330)].into_iter());
///
/// let sum = (&t).into_iter().fold(0, |s, (&k, &v)| s + k + v);
/// assert_eq!(sum, 656);
/// ```
impl<'a, K: Ord, V> IntoIterator for &'a TreapMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        Iter {
            nodes: match self.root {
                None => Vec::new(),
                Some(ref n) => vec![&**n]
            }
        }
    }
}

/// Return an mutable iterator over keys and values in the treap. The order is arbitrary.
///
/// ```
/// let mut t = treap::TreapMap::new();
/// t.extend(vec![(1, 200), (2, 120), (3, 330)].into_iter());
///
/// for (k, v) in &mut t {
///     *v += *k;
/// }
/// assert_eq!(t.get(&2), Some(&122));
/// ```
impl<'a, K: Ord, V> IntoIterator for &'a mut TreapMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> IterMut<'a, K, V> {
        IterMut {
            nodes: match self.root {
                None => Vec::new(),
                Some(ref mut n) => vec![&mut **n]
            }
        }
    }
}

impl<'a, K: Ord, V> Index<&'a K> for TreapMap<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl<'a, K: Ord, V> IndexMut<&'a K> for TreapMap<K, V> {
    fn index_mut(&mut self, key: &K) -> &mut V {
        self.get_mut(key).expect("no entry found for key")
    }
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

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        match self.nodes.pop() {
            None => None,
            Some(node) => {
                if let Some(boxed) = node.left.as_mut() {
                    self.nodes.push(&mut **boxed);
                }
                if let Some(boxed) = node.right.as_mut() {
                    self.nodes.push(&mut **boxed);
                }
                Some((&node.key, &mut node.value))
            }
        }
    }
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

impl<'a, K, V> Iterator for OrderedIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        use self::Traversal::{Left, Right};
        loop {
            match self.nodes.pop() {
                None => return None,
                Some(Left(node)) => {
                    self.nodes.push(Right(node));
                    if let Some(ref node_box) = node.left {
                        self.nodes.push(Left(&**node_box));
                    }
                }
                Some(Right(node)) => {
                    if let Some(ref node_box) = node.right {
                        self.nodes.push(Left(&**node_box));
                    }
                    return Some((&node.key, &node.value));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TreapMap;

    #[test]
    fn test_len() {
        let mut t = TreapMap::new();
        assert_eq!(t.len(), 0);
        t.insert(1, 1);
        assert_eq!(t.len(), 1);
        t.insert(1, 2);
        assert_eq!(t.len(), 1);
        t.insert(2, 2);
        t.insert(3, 3);
        assert_eq!(t.len(), 3);
        t.remove(&2);
        assert_eq!(t.len(), 2);
    }
}
