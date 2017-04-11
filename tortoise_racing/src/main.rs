fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        None
    } else {
        let full_time = (g * 3600) / (v2 - v1);
        let hour = full_time / 3600;
        let minute = (full_time % 3600) / 60;
        let second = full_time - hour * 3600 - minute * 60;
        Some(vec![hour, minute, second])
    }
}

#[test]
fn basic_tests() {
  assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
  assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
  assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
  assert_eq!(race(820, 81, 550), None);
}

fn main() {
    println!("Hello, world!");
}
