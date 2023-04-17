#[allow(dead_code)]
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

#[allow(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(a) => Some(a + 1),
        None => None,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::match_test::{Coin, UsState};

    use super::plus_one;

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

    #[test]
    fn test2() {
        let a = Some(1);
        let b = plus_one(a);
        assert_eq!(b, Some(2));
        let c = None;
        let d = plus_one(c);
        assert_eq!(d, None);
    }
}

#[test]
fn test3() {
    fn a(x: i32) -> bool {
        match x {
            1 => x % 2 == 0,
            2 => x % 2 == 0,
            _ => false,
        }
    }
    assert_eq!(a(2), true);
}
