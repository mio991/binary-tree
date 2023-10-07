use std::fmt::Debug;

/// A binary tree implementation based on a slice of Option<(K, V)>
#[derive(Clone)]
pub struct BinaryTree<K, V>(Box<[Option<(K, V)>]>);

impl<K, V> BinaryTree<K, V> {
    pub fn new() -> Self {
        Self::with_capacity(8)
    }

    pub fn with_capacity(mut capacity: usize) -> Self {
        capacity = capacity.max(1);

        Self(
            std::iter::repeat_with(Default::default)
                .take(capacity)
                .collect(),
        )
    }

    pub fn capacity(&self) -> usize {
        self.0.len()
    }
}

impl<K: Debug, V: Debug> Debug for BinaryTree<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alternate = f.alternate();

        let mut map = f.debug_map();

        for entry in self.0.iter() {
            if let Some((key, value)) = entry {
                map.entry(key, value);
            } else if alternate {
                map.entry(&Option::<K>::default(), &Option::<V>::default());
            }
        }

        map.finish()
    }
}

///   
///                 E0
///         +-------+-------+
///         E1              E2
///     +---+---+       +---+---+
///     E3      ()      E5      E6
///   +-+-+   +-+-+   +-+-+   +-+-+
///   ()  E8  ()  ()  E11 E12 ()  E14
///
/// +----+----+----+----+----+----+----+----+----+----+----+-----+-----+----+-----+
/// | E0 | E1 | E2 | E3 | () | E5 | E6 | () | E8 | () | () | E11 | E12 | () | E14 |
/// +----+----+----+----+----+----+----+----+----+----+----+-----+-----+----+-----+
impl<K, V> BinaryTree<K, V>
where
    K: Ord,
{
    fn find_index(&self, key: &K) -> usize {
        let Self(mem) = self;

        let mut index = 0;

        while let Some(
            // the location exists
            Some(
                // and there is something
                (r_key, _),
            ),
        ) = mem.get(index)
        {
            if r_key == key {
                // Found Entry
                break;
            } else {
                // Walk further

                index = if key < r_key {
                    BiTree::left(index)
                } else {
                    BiTree::right(index)
                }
            }
        }

        index
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.find_index(&key);

        if let Some(cell) = self.0.get_mut(index) {
            let result = cell.replace((key, value)).map(|kv| kv.1);

            // TODO: check balance

            result
        } else {
            self.grow();

            self.insert(key, value)
        }
    }

    fn grow(&mut self) {
        let new_capacity = self.capacity() * 2;

        self.0 = self
            .0
            .iter_mut() // We have to do iter_mut to move everything
            .map(Option::take) // We move out of old_inner
            .chain(std::iter::repeat_with(Default::default))
            .take(new_capacity)
            .collect();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.find_index(key);

        if let Some(cell) = self.0.get(index) {
            cell.as_ref().map(|kv| &kv.1)
        } else {
            None
        }
    }
}

mod BiTree {
    pub fn is_right(index: usize) -> bool {
        index % 2 == 0
    }

    pub fn parrent(index: usize) -> Option<usize> {
        if index > 0 {
            Some(if is_right(index) {
                (index - 2) / 2
            } else {
                (index - 1) / 2
            })
        } else {
            None
        }
    }

    pub fn right(index: usize) -> usize {
        index * 2 + 2
    }

    pub fn left(index: usize) -> usize {
        index * 2 + 1
    }
}

impl<K, V> IntoIterator for BinaryTree<K, V> {
    type Item = (K, V);
    type IntoIter = BinaryTreeIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            indexer: BiTreeIndexIter::new(self.capacity()),
            tree: self,
        }
    }
}

///
/// Iterates over a BinaryTree in order.
///
pub struct BinaryTreeIter<K, V> {
    tree: BinaryTree<K, V>,
    indexer: BiTreeIndexIter,
}

impl<K, V> Iterator for BinaryTreeIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.indexer.next() {
            // SAFETY: BiTreeIndexIter is limited to the capacity of tree.0
            if let Some(res) = unsafe { self.tree.0.get_unchecked_mut(index) }.take() {
                return Some(res);
            }
        }
        None
    }
}

struct BiTreeIndexIter {
    capacity: usize,
    stack: Vec<usize>,
    current: Option<usize>,
}

impl BiTreeIndexIter {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            stack: Vec::new(),
            current: Some(0),
        }
    }

    fn left(&self, node: usize) -> Option<usize> {
        let index = BiTree::left(node);

        if index < self.capacity {
            Some(index)
        } else {
            None
        }
    }

    fn right(&self, node: usize) -> Option<usize> {
        let index = BiTree::right(node);

        if index < self.capacity {
            Some(index)
        } else {
            None
        }
    }
}

impl Iterator for BiTreeIndexIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(node) = self.current {
                self.stack.push(node);
                self.current = self.left(node);
            } else if let Some(node) = self.stack.pop() {
                self.current = self.right(node);
                return Some(node);
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut b_tree = BinaryTree::with_capacity(8);

        b_tree.insert(7, "sieben");
        b_tree.insert(4, "vier");
        b_tree.insert(2, "zwei");
        b_tree.insert(5, "fünf");

        assert_eq!(b_tree.get(&2), Some("zwei").as_ref());
        assert_eq!(b_tree.get(&5), Some("fünf").as_ref());
    }

    #[test]
    fn balancing(){
        let mut b_tree = BinaryTree::with_capacity(8);

        b_tree.insert(1, "eins");
        println!("{:#?}", b_tree);
        b_tree.insert(2, "zwei");
        println!("{:#?}", b_tree);
        b_tree.insert(3, "drei");
        println!("{:#?}", b_tree);
        b_tree.insert(4, "vier");
        println!("{:#?}", b_tree);
        b_tree.insert(5, "fünf");
        println!("{:#?}", b_tree);
        b_tree.insert(6, "sechs");
        println!("{:#?}", b_tree);
        b_tree.insert(7, "sieben");
        println!("{:#?}", b_tree);

        
        let vec: Vec<_> = b_tree.into_iter().map(|kv| kv.0).collect();

        assert_eq!(vec, vec![1,2,3,4,5,6,7])

    }

    #[test]
    fn it_grows() {
        let mut b_tree = BinaryTree::with_capacity(2);

        b_tree.insert(7, "sieben");
        b_tree.insert(4, "vier");
        b_tree.insert(2, "zwei");
        b_tree.insert(5, "fünf");

        assert_eq!(b_tree.get(&2), Some("zwei").as_ref());
        assert_eq!(b_tree.get(&5), Some("fünf").as_ref());
    }

    #[test]
    fn iter_it() {
        let mut b_tree = BinaryTree::with_capacity(32);

        b_tree.insert(7, "sieben");
        b_tree.insert(4, "vier");
        b_tree.insert(2, "zwei");
        b_tree.insert(5, "fünf");

        let vec: Vec<_> = b_tree.into_iter().map(|kv| kv.0).collect();

        assert_eq!(vec, vec![2, 4, 5, 7])
    }
}
