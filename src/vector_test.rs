#[cfg(test)]
pub mod tests {
    #[test]
    fn test() {
        let v = vec![1, 2, 3, 4, 5];
        let b = v.get(3);
        if let Some(x) = b {
            println!("x={}", x);
        }
        for (i, item) in v.iter().enumerate() {
            println!("item[{}]={}", i, item);
        }

        //println!("{:?}", v);
        assert_eq!(Some(&4), b);
        let mut a = 1;
        let b = &mut a;
        *b = *b + 1;
        assert_eq!(a, 2);

        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        for item in &mut v {
            *item = *item * 2;
        }
        println!("{:?}", v);
    }
}
