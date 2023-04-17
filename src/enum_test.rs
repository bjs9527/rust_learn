pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}
#[cfg(test)]
pub mod tests {
    #[test]
    fn test_message() {
        assert_eq!(1, 1);
    }
}
