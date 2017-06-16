fn fcn_back(n: i32) -> i64 {
    fn fcn_iter(un: i64, un1: i64, count: i32) -> i64 {
        if count == 1 {
            return un1
        }
        let un2 = (6 * un * un1) / (5 * un - un1);
        fcn_iter(un1, un2, count-1)
    }
    fcn_iter(1, 2, n)
}

fn fcn_back2(n: i32) -> i64 {
    let x: i64 = 2;
    x.pow(n as u32)
}

fn fcn(n: i32) -> i64 {
    1 << n
}

fn testequal(n: i32, exp: i64) -> () {
    assert_eq!(exp, fcn(n))
}

#[test]
fn basics() {
    testequal(17, 131072);
    testequal(21, 2097152);
}
