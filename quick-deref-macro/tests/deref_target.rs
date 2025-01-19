extern crate quick_deref_macro;
use core::any::TypeId;
use core::ops::Deref;
use quick_deref_macro::quick_deref;
#[quick_deref]
pub struct SomeNewType(u8);
#[quick_deref(handle)]
pub struct WrappedHandle {
    handle: u128,
}
#[quick_deref(target = buffer)]
pub struct WrappedManyThings {
    buffer: std::sync::Arc<[u8]>,
    _handle: WrappedHandle,
}
#[test]
fn main() {
    assert_eq!(
        TypeId::of::<u8>(),
        TypeId::of::<<SomeNewType as Deref>::Target>()
    );
    assert_eq!(
        TypeId::of::<u128>(),
        TypeId::of::<<WrappedHandle as Deref>::Target>()
    );
    assert_eq!(
        TypeId::of::<std::sync::Arc<[u8]>>(),
        TypeId::of::<<WrappedManyThings as Deref>::Target>()
    );
}
