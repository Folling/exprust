pub use self::hierarchy::parse as parse;
pub use self::hierarchy::{NONE as NONE, BIN as BIN, HEX as HEX, OCT as OCT, AS_DEGREES as AS_DEGREES};
pub use self::comparison::evaluate as evaluate;

pub mod numbers;
pub mod comparison;
pub mod hierarchy;
