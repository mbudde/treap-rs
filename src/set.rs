use map::TreapMap;

/// A set based on a randomized treap
pub struct TreapSet<T> {
    map: TreapMap<T, ()>,
}

impl<T: Ord> TreapSet<T> {

    /// Returns a new empty set.
    ///
    /// ```
    /// let mut s = treap::TreapSet::new();
    /// assert_eq!(s.len(), 0);
    /// s.insert(5);
    /// ```
    pub fn new() -> TreapSet<T> {
        TreapSet {
            map: TreapMap::new(),
        }
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize { self.map.len() }

    /// Remove all elements from the set.
    pub fn clean(&mut self) { self.map.clear() }

    /// Returns true if the set is empty.
    pub fn is_empty(&self) -> bool { self.map.is_empty() }

    /// Returns true if the item is in the set.
    pub fn contains(&self, item: &T) -> bool {
        self.map.get(item).is_some()
    }

    /// Add a item to the set. Returns true if the item was not in the set already.
    pub fn insert(&mut self, item: T) -> bool {
        self.map.insert(item, ()).is_none()
    }

    /// Remove a item from the set. Returns true if the item was in the set.
    pub fn remove(&mut self, item: &T) -> bool {
        self.map.remove(item).is_some()
    }
}
