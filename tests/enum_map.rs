#[macro_use]
extern crate em;

mod enum_map_tests {
    use em::EnumMap;

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

    #[test]
    fn setting_all() {
        let mut map = EnumMap::new(|_| 1);
        assert_eq!(1, map[Example::A]);
        assert_eq!(1, map[Example::B]);

        map.set_all(|_| 3);
        assert_eq!(3, map[Example::A]);
        assert_eq!(3, map[Example::B]);
    }

    #[test]
    fn fmt() {
        let map = EnumMap::new(|example| {
            match example {
                Example::A => "abc",
                Example::B => "bcd",
            }
        });

        assert_eq!(r#"{A: "abc", B: "bcd"}"#, format!("{:?}", map));
    }

    #[test]
    fn iter_mut() {
        let mut map = EnumMap::new(|_| 1);

        map.iter_mut().for_each(|(k, v)| {
            match k {
                Example::A => *v *= 2,
                Example::B => *v *= 3,
            }
        });

        assert_eq!(2, map[Example::A]);
        assert_eq!(3, map[Example::B]);
    }
}