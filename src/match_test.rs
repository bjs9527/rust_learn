pub enum UsState {
    Alabama,
    Alaska,
}
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[cfg(test)]
pub mod tests {
    use crate::match_test::{Coin, UsState};

    #[test]
    fn test() {
        let coin = Coin::Quarter(UsState::Alaska);
        let value = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => match state {
                UsState::Alabama => 26,
                UsState::Alaska => 27,
            },
        };
        assert_eq!(value, 27);
    }
}
