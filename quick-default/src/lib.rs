//! quick-default
//!
//! The goto-crate for your quick-default-needs
//!
//! Simply mark a struct quick_default,
//! provide a default value using the #[default(..)] (or omit for implicit default)
//! and voi la.
//!
//! ```
//! use quick_default::*;
//! #[quick_default]
//! struct CoolStruct {
//!     #[default(64)] a: u32,
//!     b: u16
//! }
//! let x: CoolStruct = Default::default();
//! assert_eq!(x.a, 64);
//! assert_eq!(x.b, u16::default());
//! ```
//!

pub use quick_default_macro::quick_default;
