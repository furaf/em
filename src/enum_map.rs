use smart_enum::SmartEnum;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::iter::Iterator;

#[derive(PartialEq, Clone)]
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

    pub fn val(&self, index: K) -> V
    where V: Clone
    {
        self.data[index.as_usize()].clone()
    }

    pub fn get(&self, index: K) -> &V {
        &self.data[index.as_usize()]
    }

    pub fn get_mut(&mut self, index: K) -> &mut V {
        &mut self.data[index.as_usize()]
    }

    pub fn iter(&self) -> impl Iterator<Item=(K,&V)> {
        K::values().zip(self.data.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(K, &mut V)> {
        K::values().zip(self.data.iter_mut())
    }
}

impl<K, V> Debug for EnumMap<K, V>
    where
        K: SmartEnum + Debug + Copy + PartialEq + 'static,
        V: Debug
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        // {"Ashley": "645-7689", "Daniel": "798-1364", "Robert": "956-1745", "Katie": "435-8291"}
        write!(f, "{{")?;
        let mut next_index = 1;
        for (k, v) in self.iter() {
            if next_index < K::LENGTH {
                write!(f, "{:?}: {:?}, ", k, v)?;
            } else {
                write!(f, "{:?}: {:?}}}", k, v)?;
            }
            next_index += 1;
        }
        Ok(())
    }
}

impl<K, V> Index<K> for EnumMap<K, V>
    where K: SmartEnum + Debug + Copy + PartialEq + 'static
{
    type Output = V;

    fn index(&self, index: K) -> &V {
        self.get(index)
    }
}

impl<K, V> IndexMut<K> for EnumMap<K, V>
    where K: SmartEnum + Debug + Copy + PartialEq + 'static
{
    fn index_mut(&mut self, index: K) -> &mut V {
        self.get_mut(index)
    }
}


impl<'a, K, V> Index<&'a K> for EnumMap<K, V>
    where K: SmartEnum + Debug + Copy + PartialEq + 'static
{
    type Output = V;

    fn index(&self, index: &K) -> &V {
        self.get(*index)
    }
}

impl<'a, K, V> IndexMut<&'a K> for EnumMap<K, V>
    where K: SmartEnum + Debug + Copy + PartialEq + 'static
{
    fn index_mut(&mut self, index: &K) -> &mut V {
        self.get_mut(*index)
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
