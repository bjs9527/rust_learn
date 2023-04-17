#[allow(dead_code)]
#[cfg(test)]
pub mod tests {
    #[test]
    fn test() {
        let a = Some(1);
        if let Some(b) = a {
            assert_eq!(b, 1);
        } else {
            assert_eq!(a, None);
        }
    }
}
