use std::collections::VecDeque;

/// Maps keys of type `K` to values of type `V`.
pub struct BinaryTreeMap<K, V> {
    /*
    invariants:
    size == 0 iff tree is empty
    size == 1 iff tree is a leaf
     */
    //cached size of the tree
    size: usize,
    //the tree
    tree: BinaryTree<K, V>,
}

struct BinaryTree<K, V> {
    /*
    invariants:
    if node is None then both children are None as well
    if left child is None then right child is None as well
     */
    node: Option<Box<(K, V)>>,
    left: Option<Box<BinaryTree<K, V>>>,  // Left child
    right: Option<Box<BinaryTree<K, V>>>, // Right child
}

impl<K: PartialOrd, V> BinaryTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            tree: BinaryTree::new(),
            size: 0,
        }
    }

    #[cfg(test)]
    pub fn check_invariants(&self) {
        self.tree.check_invariants();
        //size == 0 iff node is None
        assert!(
            (self.size > 0) || self.tree.is_empty(),
            "size == 0 but tree is not empty",
        );
        assert!(
            self.tree.is_empty() || (self.size > 0),
            "tree is empty but size is > 0",
        );
        assert!(
            (self.size != 1) || self.tree.is_leaf(),
            "size == 1 but tree is not a leaf",
        );
        assert!(
            !self.tree.is_leaf() || (self.size == 1),
            "tree is a leaf but size != 1",
        );
    }

    pub fn new_leaf(key: K, value: V) -> Self {
        Self {
            tree: BinaryTree::new_leaf(key, value),
            size: 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        #[cfg(test)]
        {
            self.check_invariants();
        }
        return self.size == 0;
    }

    pub fn is_leaf(&self) -> bool {
        #[cfg(test)]
        {
            self.check_invariants();
        }
        return self.size == 1;
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        #[cfg(test)]
        {
            self.check_invariants();
        }
        self.size = self.size + 1;
        let result = self.tree.insert(key, value);
        #[cfg(test)]
        {
            self.check_invariants();
        }
        result
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.tree.get(key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        #[cfg(test)]
        {
            self.check_invariants();
        }
        if self.size > 0 {
            self.size = self.size - 1;
            let result = self.tree.remove(key);
            #[cfg(test)]
            {
                self.check_invariants();
            }
            result
        } else {
            None
        }
    }
}

impl<K: PartialOrd, V> BinaryTree<K, V> {
    pub fn new() -> Self {
        Self {
            node: None,
            left: None,
            right: None,
        }
    }

    #[cfg(test)]
    pub fn check_invariants(&self) {
        //if node is None then both left and right are None
        assert!(
            self.node.is_some() || (self.left.is_none() && self.right.is_none()),
            "node is None but left is Some",
        );
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        return self.node.is_none();
    }

    #[cfg(test)]
    pub fn is_leaf(&self) -> bool {
        return self.node.is_some() && self.left.is_none() && self.right.is_none();
    }

    pub fn new_leaf(key: K, value: V) -> Self {
        Self {
            node: Some(Box::new((key, value))),
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.node.take() {
            None => {
                self.node = Some(Box::new((key, value)));
                None
            }
            Some(root) => {
                if key == root.0 {
                    let root_val = root.1;
                    self.node = Some(Box::new((key, value)));
                    Some(root_val)
                } else if key < root.0 {
                    self.node = Some(Box::new((root.0, root.1)));
                    match self.left.as_mut() {
                        None => {
                            self.left = Some(Box::new(BinaryTree::new_leaf(key, value)));
                            None
                        }
                        Some(t) => t.insert(key, value),
                    }
                } else {
                    self.node = Some(Box::new((root.0, root.1)));
                    match self.right.as_mut() {
                        None => {
                            self.right = Some(Box::new(BinaryTree::new_leaf(key, value)));
                            None
                        }
                        Some(t) => t.insert(key, value),
                    }
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
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

    pub fn remove(&mut self, key: &K) -> Option<V> {
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
        BinaryTreeMapIntoIterator::new(self.tree)
    }
}

pub struct BinaryTreeMapIntoIterator<K, V> {
    //depth first exploration of the tree based on stack
    //invariant (I): if the stack is not empty, the top element has no left child
    _stack: VecDeque<Box<BinaryTree<K, V>>>,
}

impl<K, V> BinaryTreeMapIntoIterator<K, V> {
    fn new(t: BinaryTree<K, V>) -> Self {
        let mut _self = Self {
            _stack: VecDeque::new(),
        };
        _self.dive_leftmost(t);
        _self
    }

    //stores the left most branch on the stack. Guarantees (I)
    fn dive_leftmost(&mut self, mut t: BinaryTree<K, V>) {
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

    #[test]
    fn map_with_strings() {
        let mut map = BinaryTreeMap::new();

        map.insert(1, String::from("Hello"));
        map.insert(4, String::from("are"));
        map.insert(2, String::from("how"));
        map.insert(5, String::from("you?"));
    }
}
