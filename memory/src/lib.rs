pub mod bit;
pub mod dff;
pub mod ram;
pub mod registers;

fn t(s: &[i32]) {
    dbg!(s);
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;
    #[test]
    fn placeholder() {
        let a = vec![1, 2, 3, 1];
        t(&a[..]);
        // panic!();
    }
}
