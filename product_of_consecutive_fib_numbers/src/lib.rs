fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut previous = 0;
    let mut current = 1;
    while previous * current < prod {
        let next = current + previous;
        previous = current;
        current = next;
    }
    if previous * current == prod {
        (previous, current, true)
    } else {
        (previous, current, false)
    }
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
