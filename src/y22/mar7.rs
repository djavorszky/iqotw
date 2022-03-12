// This is the original interview question in Cassidoo's newsletter:
//
// v-------
//
// Implement a hashmap from scratch without any existing libraries in your preferred language.
//
// A hashmap should:
//
// 1. Be empty when initialized
// 2. Have the function `put(int key, int value)` which inserts a (key, value) pair into the hashmap. If the key already exists, update the corresponding value.
// 3. Have the function `get(int key)` which returns the value to which the specified key is mapped, or -1 if there’s no mapping for the key.
// 4. Have the function `remove(key)` which removes the key and its value if it exists in the map.
//
// ^------
//
// I am modifying it a bit:
// 1. Be empty when init
// 2. Work with anythingot just int
// 3. Return Option<value> for `get(T key)`
// 4. Remove works as before

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Clone, Debug)]
struct Key {
    hash: u64,
    loc: usize,
}

impl Key {
    fn from(hash: u64, loc: usize) -> Self {
        Self { hash, loc }
    }
}

pub struct CMap<K, V>
where
    K: Hash,
    V: Copy,
{
    buckets: Vec<Vec<Key>>,
    num_buckets: u64,

    values: Vec<V>,
    free_val_locations: Vec<usize>,
    len: usize,
    _phantom: PhantomData<K>,
}

impl<K, V> Default for CMap<K, V>
where
    K: Hash + std::fmt::Debug,
    V: Copy,
{
    fn default() -> Self {
        CMap::with_capacity(50)
    }
}

impl<K, V> CMap<K, V>
where
    K: Hash + std::fmt::Debug,
    V: Copy,
{
    pub fn new() -> Self {
        CMap::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buckets: vec![vec![]; capacity],
            num_buckets: capacity as u64,
            values: Vec::with_capacity(capacity),
            free_val_locations: Vec::with_capacity(10),
            len: 0,

            _phantom: PhantomData::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn cap(&self) -> usize {
        self.values.capacity()
    }

    pub fn get(&self, key: K) -> Option<V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash = hasher.finish();

        let key_loc = hash % self.num_buckets;

        self.buckets.get(key_loc as usize).and_then(|keys| {
            keys.iter().find_map(|k| {
                if k.hash == hash {
                    Some(self.values[k.loc])
                } else {
                    None
                }
            })
        })
    }

    pub fn insert(&mut self, key: K, value: V) {
        // Hash the key and get the location in bucket and in value
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash = hasher.finish();

        let key_loc = hash % self.num_buckets;
        let val_loc = if !self.free_val_locations.is_empty() {
            self.free_val_locations.pop().unwrap()
        } else {
            self.values.len()
        };

        let key_ptr = Key::from(hash, val_loc);

        if let Some(existing_keys) = self.buckets.get_mut(key_loc as usize) {
            // Check if already exists, update if so.
            for key in existing_keys.iter_mut() {
                if key.hash == hash {
                    self.values[key.loc] = value;
                    return;
                }
            }
            // not found, but we have a vector, so push the key in
            existing_keys.push(key_ptr);
            self.values.push(value);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, key: K) {
        // Hash the key and get the location in bucket and in value
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash = hasher.finish();

        let key_loc = hash % self.num_buckets;

        if let Some(existing_keys) = self.buckets.get_mut(key_loc as usize) {
            // Check if already exists, remove if so.
            for (idx, key) in existing_keys.iter_mut().enumerate() {
                if key.hash == hash {
                    self.values.remove(key.loc);
                    self.free_val_locations.push(key.loc);
                    existing_keys.remove(idx);
                    self.len -= 1;
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod cmap_tests {

    use super::*;

    #[test]
    fn empty_when_initialized() {
        let m: CMap<isize, isize> = CMap::new();

        assert_eq!(m.len(), 0);
        assert_eq!(m.cap(), 50);
    }

    #[test]
    fn capacity() {
        let m: CMap<isize, isize> = CMap::with_capacity(100);

        assert_eq!(m.len(), 0);
        assert_eq!(m.cap(), 100);
    }

    mod insert {
        use super::*;

        #[test]
        fn new_value() {
            let mut m = CMap::new();

            m.insert(1551, 2000);

            assert_eq!(m.len(), 1);

            let res = m.get(1551);

            assert!(res.is_some());
            assert_eq!(res.unwrap(), 2000);
        }

        #[test]
        fn update_existing() {
            let mut m = CMap::new();

            m.insert(1551, 2000);

            m.insert(1551, 3000);

            assert_eq!(m.len(), 1);

            let res = m.get(1551);

            assert!(res.is_some());
            assert_eq!(res.unwrap(), 3000);
        }
    }

    mod get {

        use super::*;

        #[test]
        fn returns_none_when_missing() {
            let m: CMap<isize, isize> = CMap::new();

            assert!(m.get(1).is_none());
        }

        #[test]
        fn returns_val() {
            let mut m: CMap<isize, isize> = CMap::new();

            m.insert(10, 15);

            let res = m.get(10);
            assert!(res.is_some());
            assert_eq!(res.unwrap(), 15);
        }
    }

    mod remove {
        use super::*;

        #[test]
        fn non_existent_key() {
            let mut m: CMap<isize, isize> = CMap::new();

            m.insert(10, 15);

            assert_eq!(m.len(), 1);

            m.remove(200);
            assert_eq!(m.len(), 1);
        }

        #[test]
        fn existing_key() {
            let mut m: CMap<isize, isize> = CMap::new();

            m.insert(10, 15);

            assert_eq!(m.len(), 1);

            m.remove(10);
            assert_eq!(m.len(), 0);

            assert!(m.get(10).is_none())
        }
    }
}
