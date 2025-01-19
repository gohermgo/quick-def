use quick_default_macro::quick_default;
#[quick_default]
pub struct Something {
    #[default(0)]
    pub field_1: u8,
    #[default(1000)]
    pub field_2: u16,
    pub field_3: u32,
}
fn main() {
    let x = Something::default();
    assert_eq!(x.field_2, 1000);
    assert_eq!(x.field_3, u32::default());
}
