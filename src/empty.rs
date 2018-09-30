use super::SmartEnum;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum Empty {}

impl SmartEnum for Empty {
    const LENGTH: usize = 0;
    type ValuesType = ::std::iter::Empty<Empty>;

    fn values() -> Self::ValuesType {
        ::std::iter::empty()
    }

    fn as_usize(&self) -> usize {
        unreachable!("as_usize called on Empty")
    }
}
