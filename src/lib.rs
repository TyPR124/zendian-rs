//! Endian-aware integers for obvious, type-driven conversions.
//!
//! # Features
//!
//! [`bytemuck`]: Implements [`Pod`] from [`bytemuck`].
//!
//! [`bytemuck`]: https://docs.rs/bytemuck
//! [`Pod`]: https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html
#![no_std]
#![forbid(unsafe_code)]
#![allow(non_camel_case_types)]

#[macro_use]
mod macros;

pub mod prelude {
    //! Includes all [`be`](crate::be) and [`le`](crate::le) types.
    pub use crate::be::*;
    pub use crate::le::*;
}

pub mod be {
    //! Big Endian integers
    use crate::prelude::*;
    define_endian_types! {
        u16be  u16  from_be to_be to_le u16le,
        u32be  u32  from_be to_be to_le u32le,
        u64be  u64  from_be to_be to_le u64le,
        u128be u128 from_be to_be to_le u128le,
        i16be  i16  from_be to_be to_le i16le,
        i32be  i32  from_be to_be to_le i32le,
        i64be  i64  from_be to_be to_le i64le,
        i128be i128 from_be to_be to_le i128le,
    }
}
pub mod le {
    //! Little Endian integers
    use crate::prelude::*;
    define_endian_types! {
        u16le  u16  from_le to_le to_be u16be,
        u32le  u32  from_le to_le to_be u32be,
        u64le  u64  from_le to_le to_be u64be,
        u128le u128 from_le to_le to_be u128be,
        i16le  i16  from_le to_le to_be i16be,
        i32le  i32  from_le to_le to_be i32be,
        i64le  i64  from_le to_le to_be i64be,
        i128le i128 from_le to_le to_be i128be,
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn test_be_repr() {
        assert_eq!(u16be::new(42).bits(), 42u16.to_be());
        assert_eq!(u32be::new(42).bits(), 42u32.to_be());
        assert_eq!(u64be::new(42).bits(), 42u64.to_be());
        assert_eq!(u128be::new(42).bits(), 42u128.to_be());
        assert_eq!(i16be::new(42).bits(), 42i16.to_be());
        assert_eq!(i32be::new(42).bits(), 42i32.to_be());
        assert_eq!(i64be::new(42).bits(), 42i64.to_be());
        assert_eq!(i128be::new(42).bits(), 42i128.to_be());
    }
    #[test]
    fn test_le_repr() {
        assert_eq!(u16le::new(42).bits(), 42u16.to_le());
        assert_eq!(u32le::new(42).bits(), 42u32.to_le());
        assert_eq!(u64le::new(42).bits(), 42u64.to_le());
        assert_eq!(u128le::new(42).bits(), 42u128.to_le());
        assert_eq!(i16le::new(42).bits(), 42i16.to_le());
        assert_eq!(i32le::new(42).bits(), 42i32.to_le());
        assert_eq!(i64le::new(42).bits(), 42i64.to_le());
        assert_eq!(i128le::new(42).bits(), 42i128.to_le());
    }
    #[cfg(feature = "bytemuck")]
    #[test]
    fn test_bytemuck_usage() {
        assert_eq!(bytemuck::bytes_of(&u32be::new(42)), 42u32.to_be_bytes());
        assert_eq!(bytemuck::bytes_of(&u32le::new(42)), 42u32.to_le_bytes());
        assert_eq!(bytemuck::bytes_of(&i32be::new(-42)), (-42i32).to_be_bytes());
        assert_eq!(bytemuck::bytes_of(&i32le::new(-42)), (-42i32).to_le_bytes());

        bytemuck::bytes_of_mut(&mut u32be::new(42));
    }
}
