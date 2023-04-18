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
    #[test]
    fn test2() {
        let s = String::from("我是一只小小鸟");
        let len = s.len();
        assert_eq!(len, 21);
    }

    #[test]
    fn test3() {
        let s = "你是我的小小呀小苹果";
        let chars = s.chars();
        
        //assert_eq!('你', chars[1]);
        for char in chars{
            println!("{char}");
        }
    }
}
