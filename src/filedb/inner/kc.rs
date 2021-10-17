use std::rc::Rc;

const CACHE_SIZE: usize = 128;

#[derive(Debug)]
struct KeyCacheBean<KT> {
    pub key_string: Rc<KT>,
    key_offset: u64,
    uses: u64,
}

impl<KT> KeyCacheBean<KT> {
    fn new(key_offset: u64, key_string: Rc<KT>) -> Self {
        Self {
            key_offset,
            key_string,
            uses: 0,
        }
    }
}

#[derive(Debug)]
pub struct KeyCache<KT> {
    cache: Vec<KeyCacheBean<KT>>,
}

impl<KT> KeyCache<KT> {
    pub fn new() -> Self {
        Self {
            cache: Vec::with_capacity(CACHE_SIZE),
        }
    }
}

impl<KT> Default for KeyCache<KT> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait KeyCacheTrait<KT> {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn len(&self) -> usize;
    fn get(&mut self, offset: &u64) -> Option<Rc<KT>>;
    fn put(&mut self, offset: &u64, key: KT) -> Option<Rc<KT>>;
    fn delete(&mut self, offset: &u64);
    fn clear(&mut self);
}

impl<KT> KeyCacheTrait<KT> for KeyCache<KT> {
    fn len(&self) -> usize {
        self.cache.len()
    }
    fn get(&mut self, offset: &u64) -> Option<Rc<KT>> {
        match self.cache.binary_search_by_key(offset, |a| a.key_offset) {
            Ok(k) => {
                let a = self.cache.get_mut(k).unwrap();
                a.uses += 1;
                Some(a.key_string.clone())
            }
            Err(_k) => None,
        }
    }
    fn put(&mut self, offset: &u64, key: KT) -> Option<Rc<KT>> {
        match self.cache.binary_search_by_key(offset, |a| a.key_offset) {
            Ok(k) => {
                let a = self.cache.get_mut(k).unwrap();
                a.uses += 1;
                a.key_string = Rc::new(key);
                Some(a.key_string.clone())
            }
            Err(k) => {
                let k = if self.cache.len() > CACHE_SIZE {
                    // all clear cache algorithm
                    self.cache.clear();
                    0
                } else {
                    k
                };
                let r = Rc::new(key);
                self.cache.insert(k, KeyCacheBean::new(*offset, r.clone()));
                Some(r)
            }
        }
    }
    fn delete(&mut self, offset: &u64) {
        match self.cache.binary_search_by_key(offset, |a| a.key_offset) {
            Ok(k) => {
                self.cache.remove(k);
            }
            Err(_k) => (),
        }
    }
    fn clear(&mut self) {
        self.cache.clear();
    }
}