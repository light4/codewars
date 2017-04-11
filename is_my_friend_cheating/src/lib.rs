use std::f64;

fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let m = m as u64;
    let mut result = Vec::new();
    let total = m * (m + 1) / 2;
    let min_a = ((m - 1) * m / 2) / (m + 1);
    let max_a = f64::sqrt(total as f64 + 1.0) as u64;
    for a in min_a..max_a {
        let b = (total - a) / (a + 1);
        if a * b + a + b == total {
            result.push((a as i32, b as i32));
            result.push((b as i32, a as i32));
        }
    }
    result.sort_by(|a, b| a.cmp(b));
    result
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {
    
    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);
    testing(1000003, vec![(550320, 908566), (559756, 893250), (893250, 559756), (908566, 550320)]);
}
