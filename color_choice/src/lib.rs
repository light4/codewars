fn check_choose(m: u64, n: u64) -> i64 {
    let mut result: i64 = -1;
    let h = (n + 2) / 2;

    for x in 1..h {
        if get_choices(n, x) == m {
            result = x as i64;
            break;
        }
    }
    result
}

// fn get_choices (n: u64, x: u64) -> u64 {
//     if x == 1 {
//         n
//     } else {
//         get_choices(n, x - 1) * (n - x + 1) / x
//     }
// }

fn get_choices(n: u64, x: u64) -> u64 {
    fn get_choices_iter(n:u64, x: u64, c: u64, r: u64) -> u64 {
        let r = r * (n - c + 1) / c;
        if c < x {
            get_choices_iter(n, x, c + 1, r)
        } else {
            r
        }
    }
    get_choices_iter(n, x, 1, 1)
}

fn dotest(m: u64, n: u64, exp: i64) -> () {
    assert_eq!(check_choose(m, n), exp)
} 
#[test]
fn basics_check_choose() {
    dotest(6, 4, 2);
    dotest(4, 4, 1);
    dotest(35, 7, 3);
    dotest(36, 7, -1);
    dotest(184756, 20, 10);
}
