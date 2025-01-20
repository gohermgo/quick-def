//! # quick-default
//!
//! The goto-crate for your quick-default-needs
//! This crate aims to provide developers with an ergonomic
//! alternative to writing a default implementation.
//!
//! Simply mark a struct quick_default,
//! provide a default value using the #[default(..)] (or omit for implicit default)
//! and voi la.
//!
//! # Examples
//!
//! A simple default for any struct can be added (without the need for an implementation block)
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
