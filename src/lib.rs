pub fn OwOify(input_text: String, level: OwoLevel) -> String {
  match level {
    OwoLevel::Low => {

      "low return".parse().unwrap()
    },
    OwoLevel::Medium => {

      "medium return".parse().unwrap()
    },
    OwoLevel::High => {

      "high return".parse().unwrap()
    }
  }
}

pub enum OwoLevel {
    Low,
    Medium,
    High
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testowo() {
      assert_eq!(OwOify(String::from("hello"), OwoLevel::Low), String::from("low return"));
      assert_eq!(OwOify(String::from("hello"), OwoLevel::Medium), String::from("medium return"));
      assert_eq!(OwOify(String::from("hello"), OwoLevel::High), String::from("high return"));
    }
}
