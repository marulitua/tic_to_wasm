pub mod models;

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return 14
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
