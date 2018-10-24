#[macro_use]
#[allow(unused_imports)]
extern crate smart_enum_derive;
#[doc(hidden)]
pub use self::smart_enum_derive::*;

extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;


mod enum_map;
mod smart_enum;
mod empty;

pub use enum_map::*;
pub use smart_enum::*;
pub use empty::*;
