use rand;

pub fn add_one(input: i32) -> i32 {
    input + 1
}

// pub fn r() -> i32

#[cfg(test)]
mod test {
    use crate::add_one;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}
