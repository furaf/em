use itertools::Itertools;
use itertools::Product;

use std::iter::Iterator;
use std::iter::once;
use std::iter::Chain;
use std::iter::Once;
use std::iter::Map;

pub trait SmartEnum: Clone + 'static {
    const LENGTH: usize;
    type ValuesType: Iterator<Item=Self> + Clone;

    fn values() -> Self::ValuesType;
    fn as_usize(&self) -> usize;
}


impl <A: SmartEnum, B: SmartEnum> SmartEnum for (A, B) {
    const LENGTH: usize = A::LENGTH * B::LENGTH;
    type ValuesType = Product<A::ValuesType, B::ValuesType>;

    fn values() -> Self::ValuesType {
        A::values().cartesian_product(B::values())
    }

    fn as_usize(&self) -> usize {
        match self {
            (a, b) => B::LENGTH * a.as_usize() + b.as_usize()
        }
    }
}

fn some<T>(x: T) -> Option<T> {
    Some(x)
}

impl <A: SmartEnum> SmartEnum for Option<A> {
    const LENGTH: usize = 1 + A::LENGTH;
    type ValuesType = Chain<Once<Self>, Map<A::ValuesType, fn(A) -> Option<A>>>;

    fn values() -> Self::ValuesType {
        once(None).chain(A::values().map(some as fn(A) -> Option<A>))
    }

    fn as_usize(&self) -> usize {
        match self {
            None => 0,
            Some(x) => 1 + x.as_usize(),
        }
    }
}