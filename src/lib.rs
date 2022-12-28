#![feature(hash_drain_filter)]
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::hash::Hash;

pub struct LRUTimedCache<T: Eq+Hash+Clone, V: Copy> {
    elems: HashMap<T, Instant>,
    val: HashMap<T, V>,
    retrieval_func: fn(key: T) -> V,
    max_len: usize,
}

impl <T: Eq+Hash+Clone, V: Copy> LRUTimedCache<T, V> {
    pub fn new(max_len: usize, retrieval_func: fn(key: T) -> V) -> Self {
        Self {
            elems: HashMap::new(),
            val: HashMap:: new(),
            retrieval_func,
            max_len,
        }
    }
    pub fn insert(&mut self, key: T, val: V) {
        if self.elems.len() == self.max_len {
            self.remove_lru();
        }
        self.elems.insert(key.clone(), Instant::now());
        self.val.insert(key, val);
    }

    fn remove_lru(&mut self) {
        let val = match self.elems.values().min() {
            Some(val) => {
                val.clone()
            }, None => {
                return;
            }
        };
        let mut key = None;
        for (k, v) in self.elems.iter_mut() {
            if val == *v {
                key = Some(k.clone());
                break;
            }
        }
        self.elems.remove(&key.as_mut().unwrap());
        self.val.remove(&key.as_mut().unwrap());
    }

    pub fn get(&mut self, key: T) -> V {
        match self.val.get(&key) {
            Some(val) => {
                self.elems.insert(key.clone(), Instant::now());
                *val
            }
            None => {
                let result = (self.retrieval_func)(key.clone());
                self.insert(key, result);
                result
            },
        }
    }

    pub fn contains_key(&mut self, key: T) -> bool {
        if self.elems.contains_key(&key) {
            self.elems.insert(key, Instant::now());
            return true;
        }
        false
    }

    pub fn clear_older_than(&mut self, duration: Duration) -> usize {
        let mut keys = Vec::<T>::new();
        for (k, v) in self.elems.iter_mut() {
            if v.elapsed() > duration {
                keys.push(k.clone());
            }
        }

        self.elems.drain_filter(|k, _| keys.contains(k));
        self.val.drain_filter(|k, _| keys.contains(k));

        return keys.len();
    }

    pub fn clear(&mut self) {
        self.val.clear();
        self.elems.clear();
    }
}
