use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Cache<T>
{
    dict: HashMap<u64, Vec<T>>,
    associativity: u64,
    size: u64,
    num_sets: u64,
    policy: eviction_policy
}

#[derive(Default)]
pub enum eviction_policy
{
    #[default]
    LRU,
    MRU,
    LFU
}

impl<T: PartialEq> Cache<T>
{
    pub fn new(_associativity: u64, _policy: eviction_policy, _size: u64) -> Self
    {
        let mut _dict : HashMap<u64, Vec<T>> = HashMap::new();
        let _num_sets = _size / _associativity;

        for i in 0.._num_sets
        {
            _dict.insert(i, Vec::new());
        }

        Cache
        {
            dict: _dict,
            associativity: _associativity,
            size: _size,
            num_sets: _num_sets,
            policy: _policy
        }
    }

    pub fn lookup(&mut self, value: T) -> Option<T>
    {
        self.search(value)
    }

    fn find_set(&mut self, key: T) -> u64
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut key);
        hasher.finish() % self.num_sets
    }

    fn search(&mut self, key: T) -> Option<T>
    {
        let mut set = self.dict.get_mut(&self.find_set(key)).unwrap();

        match set.iter().position(|&x| x == key)
        {
            Some(index) => 
            {
                let val = set[index];
                set.remove(index);
                set.insert(0, val);
                Some(val)
            },

            None => 
            {
                if(set.len() == self.associativity as usize)
                {
                    set.pop();
                }
                set.insert(0, key);
                None
            }
        }
    }
}