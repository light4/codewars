use std::collections::BTreeSet;

fn part(n: i64) -> String {
    let products = products(n);
    let range = range(&products);
    let average = average(&products);
    let median = median(&products);
    let result = format!("Range: {} Average: {:.*} Median: {:.*}", range, 2, average, 2, median);
    result
}

fn parts_of_len_by_index(n: i64, l: i64, i: i64) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![];
    for mut test in parts_of_len((n - i), (l - 1)) {
        test.push(i);
        result.push(test);
    }
    return result
}

fn parts_of_len(n: i64, l: i64) -> Vec<Vec<i64>> {
    if l == 1 {
        return vec![vec![n]]
    } else {
        let result = (1..(n/l + 1)).map(|i| parts_of_len_by_index(n, l, i))
                                   .fold(vec![], |acc, x| {let mut r: Vec<Vec<i64>> = x; r.extend(acc); return r});
        return result
    }
}

fn partitions(n: i64) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = vec![];
    for i in 1..(n + 1) {
        result.extend(parts_of_len(n, i))
    }
    return result
}

fn products(n: i64) -> BTreeSet<i64> {
    let mut result = BTreeSet::new();
    let partitions = partitions(n);
    for item in partitions {
        let product: i64 = item.iter().product();
        result.insert(product);
    }
    result
}

fn range(ref r: &BTreeSet<i64>) -> i64 {
    r.iter().max().unwrap() - r.iter().min().unwrap()
}

fn average(ref r: &BTreeSet<i64>) -> f64 {
    let len = r.len();
    let sum: i64 = r.into_iter().sum();
    sum as f64 / len as f64
}

fn median(ref r: &BTreeSet<i64>) -> f64 {
    let n = r.len();
    let a: Vec<_> = r.into_iter().collect();
    if n % 2 == 0 {
        return (a[n / 2] + a[n / 2 - 1]) as f64 / 2.0
    } else {
        return *a[(n - 1) / 2] as f64
    }
}

fn testequal(ans: &str, sol: &str) {
    assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
}

#[test]
fn returns_expected() {
    testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
    testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
    testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
    testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
    testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
}
