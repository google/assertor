pub mod basic;
pub mod iterator;
pub mod map;
pub mod option;
pub mod result;
pub mod set;
pub mod string;
pub mod vec;

#[cfg(feature = "float")]
pub mod float;

#[cfg(any(test, doc, feature = "testing"))]
pub(crate) mod testing;
