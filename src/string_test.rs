#[cfg(test)]

mod tests {
    use std::ops::Add;

    use super::*;
    #[test]
    fn test() {
        let s = String::from("hello");
        let h = String::from("world");
        let a = s + &h;
        assert_eq!(a, "helloworld");

        let mut s = String::from("hello");
        s.push_str(" world");
        s.push('!');
        assert_eq!(s, "hello world!");
        s = s.add("!!");
        println!("{:?}", s);
    }
}
