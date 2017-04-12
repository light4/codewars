fn valid_braces(s: &str) -> bool {
  let data = vec!["{}", "()", "[]"];
  if !data.iter().any(|&x| s.contains(x)) {
    return false;
  }
  let mut braces = "";
  for b in data {
    if s.contains(b) {
      braces = b;
      break;
    }
  }
  if s == braces {
    return true;
  } else {
    return valid_braces(s.replace(braces, "").as_str());
  }
}

#[test]
fn basic_tests() {
  assert_eq!(valid_braces("()"), true);
  assert_eq!(valid_braces("[(])"), false);
}
