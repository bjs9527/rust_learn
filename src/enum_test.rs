#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
        let msg_quit = Message::Quit;
        let msg_move = Message::Move { x: 1, y: 2 };
        let msg_write = Message::Write("hello".to_string());
        let msg_change_color = Message::ChangeColor(0, 0, 0);
        assert_eq!(msg_quit, Message::Quit);
        assert_eq!(msg_move, Message::Move { x: 1, y: 2 });
        assert_eq!(msg_write, Message::Write("hello".to_string()));
        assert_eq!(msg_change_color, Message::ChangeColor(0,0,0));
    }
}
