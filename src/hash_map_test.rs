#[cfg(test)]
pub mod tests {
    use std::{ collections::HashMap};

    #[test]
    fn test() {
        let mut map = HashMap::new();
        map.insert("name", "bjs007");
        let a = map["name"];
        let b = map.get("name");
        assert_eq!(a, *b.unwrap());

        let c = map.entry("name").or_insert("bjs9527");
        println!("{:?}", c);
        assert_eq!(c, &"bjs007");
    }
    #[test]
    fn test2() {
        let str = "red green red blue yellow red green blue pink gray pink";
        let mut map = HashMap::new();
        for word in str.split_whitespace() {
          // 如果map中没有该键，则插入键，然后返回该键对应的值的可变引用。
            let count = map.entry(word).or_insert(0);
            // 获取到值的可变引用后就可以设置该值。
            *count += 1;
        }
        println!("{:?}", map);
        assert_eq!(*map.get("red").unwrap(),3);
    }
}
