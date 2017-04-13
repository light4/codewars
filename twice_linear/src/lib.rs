use std::cmp::min;
use std::collections::VecDeque;

fn dbl_linear(n: u32) -> u32{
    let mut u = 1;
    let mut p2: VecDeque<u32> = VecDeque::new();
    let mut p3: VecDeque<u32> = VecDeque::new();
    for _ in 0..n {
        p2.push_back(u * 2 + 1);
        p3.push_back(u * 3 + 1);
        u = min(p2[0], p3[0]);
        if u == p2[0] {
            p2.pop_front();
        }
        if u == p3[0] {
            p3.pop_front();
        }
    }
    u
}

fn testing(n: u32, exp: u32) -> () {
    assert_eq!(dbl_linear(n), exp)
}

#[test]
fn basics_dbl_linear() {
    testing(10, 22);
    testing(20, 57);
    testing(30, 91);
    testing(50, 175);
    testing(100, 447);
}
