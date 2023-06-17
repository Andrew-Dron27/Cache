use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

trait Cache<T>
{
    fn new(_associativity: u64, _size: u64) -> Self;
    fn lookup(&mut self, target: T) -> Option<&T>;
}

pub struct LruCache<'a, T>
{
    dict: HashMap<u64, Vec<&'a T>>,
    associativity: u64,
    size: u64,
    num_sets: u64,
}

pub struct LFU_Cache<T>
{
    dict: HashMap<u64, Vec<T>>,
    associativity: u64,
    size: u64,
    num_sets: u64,
}

impl<'a, T: PartialEq + Hash> Cache<T> for LruCache<'a,T>
{
    fn new(_associativity: u64, _size: u64) -> Self
    {
        let mut _dict : HashMap<u64, Vec<&T>> = HashMap::new();
        let _num_sets = _size / _associativity;

        for i in 0.._num_sets
        {
            _dict.insert(i, Vec::new());
        }

        LruCache
        {
            dict: _dict,
            associativity: _associativity,
            size: _size,
            num_sets: _num_sets,
        }
    }

    fn lookup(&mut self, value: T) -> Option<&T>
    {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let loc  = hasher.finish() % self.num_sets;

        let mut set = self.dict.get_mut(&loc).unwrap();

        match set.iter().position(|&x| *x == value)
        {
            Some(index) => 
            {
                let val = set[index];
                set.remove(index);
                set.insert(0, &val);
                Some(val)
            },

            None => 
            {
                if(set.len() == self.associativity as usize)
                {
                    set.pop();
                }
                set.insert(0, &value);
                None
            }
        }
    }

}