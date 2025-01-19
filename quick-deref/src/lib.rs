//! The quick_deref macro
//!
//! A proc-macro for shrinking boiler-plate surface-area.
//!
//! ```
//! use quick_deref::*;
//! #[quick_deref]
//! pub struct SomeNewType(u8)
//! assert_eq!(
//!        core::any::TypeId::of::<u8>(),
//!        core::any::TypeId::of::<<SomeNewType as core::ops::Deref>::Target>()
//! );
//! ```
//!
//! Alternatively,
//! ```
//! use quick_deref::*;
//! #[quick_deref(handle)]
//! pub struct ComplicatedObject {
//!     handle: u128
//! }
//! assert_eq!(
//!     core::any::TypeId::of::<u128>(),
//!     core::any::TypeIf::of::<<ComplicatedObject as core::ops::Deref>::Target>()
//! );
//! ```

pub use quick_deref_macro::quick_deref;
