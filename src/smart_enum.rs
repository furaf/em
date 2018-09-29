pub trait SmartEnum: Clone {
    const LENGTH: usize;

    fn values() -> &'static [Self];

    fn as_usize(&self) -> usize;
}

impl <A: SmartEnum, B: SmartEnum> SmartEnum for (A, B) {
    const LENGTH: usize = A::LENGTH * B::LENGTH;

    fn values() -> &'static [Self] {
        unimplemented!("Please implement.") // TODO[typesafe]: do it.
    }

    fn as_usize(&self) -> usize {
        match self {
            (a, b) => B::LENGTH * a.as_usize() + b.as_usize()
        }
    }
}

impl <A: SmartEnum> SmartEnum for Option<A> {
    const LENGTH: usize = 1 + A::LENGTH;

    fn values() -> &'static [Self] {
        unimplemented!("Please implement.") // TODO[typesafe]: do it.
    }

    fn as_usize(&self) -> usize {
        match self {
            None => 0,
            Some(x) => 1 + x.as_usize(),
        }
    }
}