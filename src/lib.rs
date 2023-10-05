use std::fmt::Debug;

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
///     E3      E4      E5      E6
///   +-+-+   +-+-+   +-+-+   +-+-+
///   ()  E8  ()  ()  E11 E12 ()  E14
///
/// +----+----+----+----+----+----+----+----+----+----+----+-----+-----+----+-----+
/// | E0 | E1 | E2 | E3 | E4 | E5 | E6 | () | E8 | () | () | E11 | E12 | () | E14 |
/// +----+----+----+----+----+----+----+----+----+----+----+-----+-----+----+-----+
impl<K, V> BinaryTree<K, V>
where
    K: Ord,
    K: Debug,
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
                    index * 2 + 1
                } else {
                    index * 2 + 2
                }
            }
        }

        index
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.find_index(&key);

        if let Some(cell) = self.0.get_mut(index) {
            cell.replace((key, value)).map(|kv| kv.1)
        } else {
            // Either rebalance or grow
            todo!()
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut b_tree = BinaryTree::with_capacity(32);

        b_tree.insert(7, "sieben");
        b_tree.insert(4, "vier");
        b_tree.insert(2, "zwei");
        b_tree.insert(5, "fünf");

        assert_eq!(b_tree.get(&2), Some("zwei").as_ref());
        assert_eq!(b_tree.get(&5), Some("fünf").as_ref());

        println!("{:?}", &b_tree);

        b_tree.insert(6, "sechs");

        println!("{:?}", &b_tree);
    }
}
