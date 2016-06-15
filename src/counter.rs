use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Counter<T> {
    inner: HashMap<T, u64>,
    total: u64,
}

impl<T: Hash + Eq> Counter<T> {
    pub fn new() -> Self {
        Counter { inner: HashMap::new(), total: 0 }
    }

    pub fn increment(&mut self, key: T) {
        let counter = self.inner.entry(key).or_insert(0);
        *counter += 1;
        self.total += 1;
    }

    pub fn get(&self, key: T) -> u64 {
        self.inner.get(&key).cloned().unwrap_or(0)
    }

    pub fn total(&self) -> u64 { self.total }

    pub fn counts(&self) -> Counts<T> {
        Counts { inner: self.inner.iter() }
    }
}

pub struct Counts<'a, K: 'a> {
    inner: hash_map::Iter<'a, K, u64>,
}

impl<'a, K> Iterator for Counts<'a, K> {
    type Item = (&'a K, &'a u64);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

