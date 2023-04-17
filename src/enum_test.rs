#[derive(PartialEq, Eq, PartialOrd, Ord,Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}
#[cfg(test)]
pub mod tests {
    use super::Message;

    #[test]
    fn test_message() {
        let msg = Message::Quit;
        assert_eq!(msg, Message::Quit);
    }
}
