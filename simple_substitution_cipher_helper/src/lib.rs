struct Cipher {
  map1: String,
  map2: String,
}

impl Cipher {
  fn new(map1: &str, map2: &str) -> Cipher {
      Cipher {
          map1: map1.to_owned(),
          map2: map2.to_owned(),
      }
  }
  
  fn encode(&self, string: &str) -> String {
      let mut new_string = String::new();
      for c in string.chars() {
          let index = self.map1.find(c);
          match index {
              Some(n) => new_string.push(self.map2.chars().nth(n).unwrap()),
              None => new_string.push(c),
          }
      }
      new_string
  }
  
  fn decode(&self, string: &str) -> String {
      let mut new_string = String::new();
      for c in string.chars() {
          let index = self.map2.find(c);
          match index {
              Some(n) => new_string.push(self.map1.chars().nth(n).unwrap()),
              None => new_string.push(c),
          }
      }
      new_string
  }
}

#[test]
fn examples() {
  let map1 = "abcdefghijklmnopqrstuvwxyz";
  let map2 = "etaoinshrdlucmfwypvbgkjqxz";

  let cipher = Cipher::new(map1, map2);
  
  assert_eq!(cipher.encode("abc"), "eta");
  assert_eq!(cipher.encode("xyz"), "qxz");
  assert_eq!(cipher.decode("eirfg"), "aeiou");
  assert_eq!(cipher.decode("erlang"), "aikcfu");
}
