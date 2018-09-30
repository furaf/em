use smart_enum::SmartEnum;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::iter::Iterator;

#[derive(Debug, PartialEq, Clone)]
pub struct EnumMap<K, V> {
    data: Vec<V>,
    phantom: PhantomData<K>,
}

impl<K, V> EnumMap<K, V>
where
    K: SmartEnum + Debug + Copy + PartialEq + 'static
{
    pub fn new<F>(factory: F) -> EnumMap<K, V>
    where F: Fn(K) -> V, K: 'static
    {
        EnumMap {
            data: K::values().map(|e| factory(e.clone())).collect(),
            phantom: PhantomData,
        }
    }

    pub fn set_all<F>(&mut self, factory: F) where F: Fn(K) -> V, K: 'static {
        self.data = K::values().map(|e| factory(e)).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item=(K,&V)> {
        K::values().zip(self.data.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(K, &mut V)> {
        K::values().zip(self.data.iter_mut())
    }
}

impl<K, V> Index<K> for EnumMap<K, V>
where K: SmartEnum
{
    type Output = V;

    fn index(&self, index: K) -> &V {
        &self.data[index.as_usize()]
    }
}

impl<K, V> IndexMut<K> for EnumMap<K, V>
where K: SmartEnum
{
    fn index_mut(&mut self, index: K) -> &mut V {
        &mut self.data[index.as_usize()]
    }
}


impl<'a, K, V> Index<&'a K> for EnumMap<K, V>
    where K: SmartEnum
{
    type Output = V;

    fn index(&self, index: &K) -> &V {
        &self.data[index.as_usize()]
    }
}

impl<'a, K, V> IndexMut<&'a K> for EnumMap<K, V>
    where K: SmartEnum
{
    fn index_mut(&mut self, index: &K) -> &mut V {
        &mut self.data[index.as_usize()]
    }
}


#[cfg(test)]
mod enum_map_tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, SmartEnum)]
    enum Example {
        A,
        B,
    }

    #[test]
    fn test_it() {
        let mut map = EnumMap::new(|_| 0);

        assert_eq!(0, map[Example::A]);
        assert_eq!(0, map[Example::B]);
        map[Example::A] = 1;

        assert_eq!(1, map[Example::A]);
        assert_eq!(0, map[Example::B]);

        let mut iter = map.iter();
        assert_eq!(Some((Example::A, &1)), iter.next());
        assert_eq!(Some((Example::B, &0)), iter.next());
        assert_eq!(None, iter.next());
    }
}
