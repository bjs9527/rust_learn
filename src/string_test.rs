#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let s = String::from("hello");
        let h = String::from("world");
        let a = s + &h;
        assert_eq!(a, "helloworld");
    }
}
