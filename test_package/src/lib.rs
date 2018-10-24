#[macro_use]
extern crate em;

#[derive(SmartEnum, Clone, Copy, Debug, PartialEq)]
enum TestEnum {
   A,
   B,
}

#[cfg(test)]
mod tests {
    use em::EnumMap;

    use super::*;

    #[test]
    fn it_works() {
        let mut em = EnumMap::new(|_| 0.0);
        em[TestEnum::A] = 1.0;
//        let em = EnumMap::<TestEnum, i32>::new()(|_| 0.0);
    }
}
