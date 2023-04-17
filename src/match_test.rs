pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[cfg(test)]
pub mod tests{
    use crate::match_test::Coin;

  #[test]
  fn test(){
    let coin = Coin::Dime;
    let value=match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,        
    };
    assert_eq!(value,10);
  }
}