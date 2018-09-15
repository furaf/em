pub trait SmartEnum: Clone {
    const LENGTH: usize;

    fn values() -> &'static [Self];

    fn as_usize(&self) -> usize;
}
