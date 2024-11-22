use std::collections::VecDeque;

/// Maps keys of type `K` to values of type `V`.
pub struct BinaryTreeMap<K, V> {
    // TODO: add struct members,
    node: Option<Box<(K, V)>>,
    left: Option<Box<BinaryTreeMap<K, V>>>,  // Left child
    right: Option<Box<BinaryTreeMap<K, V>>>, // Right child
}

/// Hint: you may need to define additional structs/enums,
/// or add trait bounds, etc.
impl<K: PartialOrd + Copy, V: Copy> BinaryTreeMap<K, V> {
    pub fn new() -> Self {
        //todo!("Create a new empty map")
        Self {
            node: None,
            left: None,
            right: None,
        }
    }

    pub fn new_leaf(key: K, value: V) -> Self {
        //todo!("Create a new empty map")
        Self {
            node: Some(Box::new((key, value))),
            left: None,
            right: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        //todo!("Whether the map is empty")
        return self.node.is_none();
    }

    pub fn is_leaf(&self) -> bool {
        //todo!("Whether the map is empty")
        return self.node.is_some() && self.left.is_none() && self.right.is_none();
    }

    pub fn len(&self) -> usize {
        //todo!("Return the number of elements in the map")
        self.node.as_ref().map_or(0, |_| 1)
            + self.left.as_ref().map_or(0, |left| left.len())
            + self.right.as_ref().map_or(0, |right| right.len())
    }

    fn get_root_key(&self) -> Option<K> {
        self.node.as_ref().and_then(|x| Some(x.0))
    }

    fn get_root_val(&self) -> Option<V> {
        self.node.as_ref().and_then(|x| Some(x.1))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.get_root_key() {
            None => {
                self.node = Some(Box::new((key, value)));
                None
            }
            Some(root_key) => {
                if key == root_key {
                    let root_val = self.get_root_val();
                    self.node = Some(Box::new((key, value)));
                    root_val
                } else if key < root_key {
                    match self.left.as_mut() {
                        None => {
                            self.left = Some(Box::new(BinaryTreeMap::new_leaf(key, value)));
                            None
                        }
                        Some(t) => t.insert(key, value),
                    }
                } else {
                    match self.right.as_mut() {
                        None => {
                            self.right = Some(Box::new(BinaryTreeMap::new_leaf(key, value)));
                            None
                        }
                        Some(t) => t.insert(key, value),
                    }
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        //todo!("Retrieve the value associated to a key if it exists")
        if let Some(ref root) = self.node {
            if root.0 == *key {
                Some(&root.1)
            } else if root.0 > *key {
                self.left.as_ref()?.get(key)
            } else {
                self.right.as_ref()?.get(key)
            }
        } else {
            None
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        //todo!("Whether the map contains the given key")
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        //todo!("If there is a value with the given key, remove it from the map and return it")
        if let Some(root) = self.node.as_mut() {
            if *key < root.0 {
                self.left.as_mut()?.remove(key)
            } else if *key > root.0 {
                self.right.as_mut()?.remove(key)
            } else {
                self.extract_root().and_then(|t| Some(t.1))
            }
        } else {
            None
        }
    }

    fn extract_root(&mut self) -> Option<Box<(K, V)>> {
        if let Some(root) = self.node.take() {
            self.node = self
                .left
                .as_mut()
                .and_then(|t| t.extract_root())
                .or_else(|| self.right.as_mut().and_then(|t| t.extract_root()));
            Some(root)
        } else {
            None
        }
    }
}

/// Create an iterator over the (key, value) pairs of the map,
/// ordered by key.
///
/// Fix this definition !
/// It should consume the map and return owned pairs, not references!
impl<K: Ord, V> IntoIterator for BinaryTreeMap<K, V> {
    type Item = (K, V);

    type IntoIter = BinaryTreeMapIntoIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        BinaryTreeMapIntoIterator::new(self)
    }
}

pub struct BinaryTreeMapIntoIterator<K, V> {
    //depth first exploration of the tree based on stack
    //invariant (I): if the stack is not empty, the top element has no left child
    _stack: VecDeque<Box<BinaryTreeMap<K, V>>>,
}

impl<K, V> BinaryTreeMapIntoIterator<K, V> {
    fn new(t: BinaryTreeMap<K, V>) -> Self {
        let mut _self = Self {
            _stack: VecDeque::new(),
        };
        _self.dive_leftmost(t);
        _self
    }

    //stores the left most branch on the stack. Guarantees (I)
    fn dive_leftmost(&mut self, mut t: BinaryTreeMap<K, V>) {
        while let Some(left) = t.left.take() {
            self._stack.push_front(Box::new(t));
            t = *left;
        }
        self._stack.push_front(Box::new(t));
    }
}

impl<K, V> Iterator for BinaryTreeMapIntoIterator<K, V> {
    //when asked for some element, output the current node content
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut current) = self._stack.pop_front() {
            //invariant (I): current.left is None
            //if the current node has a right child R, dive along RLLL...L
            current.right.take().map(|right| self.dive_leftmost(*right));
            //output (key,value) pair stored on the current node
            current.node.and_then(|pair| Some((pair.0, pair.1)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_insert_contains() {
        let mut map = BinaryTreeMap::new();

        print!("insert 1 (10,12)\n");
        map.insert(1, (10, 12));
        print!("insert 4 (23,17)\n");
        map.insert(4, (23, 17));
        print!("insert 2 (1,1)\n");
        map.insert(2, (1, 1));

        assert_eq!(map.get(&0), None);
        assert_eq!(map.get(&1), Some(&(10, 12)));
        assert_eq!(map.get(&2), Some(&(1, 1)));
        assert_eq!(map.get(&3), None);
        assert_eq!(map.get(&4), Some(&(23, 17)));

        assert_eq!(map.insert(2, (15, 0)), Some((1, 1)));
    }

    #[test]
    fn map_remove() {
        let mut map = BinaryTreeMap::new();

        map.insert(1, (10, 12));
        map.insert(4, (23, 17));
        map.insert(2, (1, 1));
        map.insert(5, (1, 3));
        assert_eq!(map.len(), 4);

        assert_eq!(map.get(&4), Some(&(23, 17)));
        assert_eq!(map.remove(&4), Some((23, 17)));
        assert_eq!(map.get(&4), None);
        assert_eq!(map.insert(4, (8, 9)), None);
        map.remove(&1);
        map.remove(&2);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn map_into_iter() {
        let mut map = BinaryTreeMap::new();

        map.insert(1, "Hello");
        map.insert(4, "are");
        map.insert(2, "how");
        map.insert(5, "you?");

        let mut iter = map.into_iter();
        assert_eq!(iter.next(), Some((1, "Hello")));
        assert_eq!(iter.next(), Some((2, "how")));
        assert_eq!(iter.next(), Some((4, "are")));
        assert_eq!(iter.next(), Some((5, "you?")));
        assert_eq!(iter.next(), None);
    }
}
