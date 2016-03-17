use std::collections::HashMap;
use std::hash::Hash;
//use core::hash::Hash;

pub struct Usage<K,V> {
    num_tracked_cycles: u8,
    data: HashMap<K,UsageData<V>>,
}

impl<K: Eq + Hash,V> Usage<K,V> {
    pub fn new(cycles: u8) -> Self {
        Usage{
            num_tracked_cycles: cycles,
            data: HashMap::new(),
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.data.insert(key,UsageData::new(val)).map(|d| d.data)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.data.get_mut(key).and_then(|d| {
            d.in_progress_reqs += 1;
            Some(d)
        }).map(|d| &d.data)
    }
    
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key).and_then(|d| {
            d.in_progress_reqs += 1;
            Some(d)
        }).map(|d| &mut d.data)
    }
}

pub struct UsageData<V> {
    last_cycle_reqs: u64,
    in_progress_reqs: u64,
    data: V,
}

impl<V> UsageData<V> {
    pub fn new(data: V) -> Self {
        UsageData{
            last_cycle_reqs: 0,
            in_progress_reqs: 0,
            data: data,
        }
    }
}
