extern crate quick_def_macro;
use quick_def_macro::quick_def;

#[quick_def]
pub struct WrappedIterator {
    inner: Vec<u32>,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
