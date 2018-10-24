#[macro_use]
extern crate em;

mod smart_enum_map_tests {
    use em::SmartEnum;

    #[derive(Clone, Copy, Debug, PartialEq, SmartEnum)]
    enum Example {
        A,
        B,
    }

    #[test]
    fn values() {
        assert_eq!(vec![Example::A, Example::B], Example::values().collect::<Vec<_>>());
    }

    #[test]
    fn as_usize() {
        assert_eq!(0, Example::A.as_usize());
        assert_eq!(1, Example::B.as_usize());
    }

    #[test]
    fn length() {
        assert_eq!(2, Example::LENGTH);
    }
}