pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Self { value }
    }
}

#[cfg(test)]
pub mod tests{
    use super::Guess;

  #[test]
  #[should_panic]
  pub fn test(){
    Guess::new(200);
  }
}