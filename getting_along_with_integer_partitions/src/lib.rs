use std::collections::HashMap;

fn part(n: i64) -> String {
    let products = products(n);
    let range = range(&products);
    let average = average(&products);
    let median = median(&products);
    let result = format!("Range: {} Average: {:.*} Median: {:.*}", range, 2, average, 2, median);
    result
}

fn partitions(n: i64) -> Vec<Vec<i64>> {
    let mut cache = HashMap::new();

    fn partitions_iter(n: i64, cache: &mut HashMap<i64, Vec<Vec<i64>>>) -> Vec<Vec<i64>> {
        if cache.contains_key(&n) {
            let ref result = cache[&n];
            return result.clone()
        } else {
            let mut result = vec![vec![n]];
            for i in 1..n {
                let sub_parts = partitions_iter(n - i, cache);
                for mut sub_part in sub_parts {
                    sub_part.push(i);
                    sub_part.sort();
                    result.push(sub_part);
                }
            }
            result.sort();
            result.dedup();
            cache.insert(n, result.clone());
            return result
        }
    }

    let result = partitions_iter(n, &mut cache);
    result
}

fn products(n: i64) -> Vec<i64> {
    let mut result = vec![];
    for item in partitions(n) {
        result.push(item.iter().product())
    }
    result.sort();
    result.dedup();
    result
}

fn range(ref r: &Vec<i64>) -> i64 {
    r[r.len() - 1] - r[0]
}

fn average(ref r: &Vec<i64>) -> f64 {
    let len = r.len();
    let sum: i64 = r.into_iter().sum();
    sum as f64 / len as f64
}

fn median(ref r: &Vec<i64>) -> f64 {
    let n = r.len();
    if n % 2 == 0 {
        return (r[n / 2] + r[n / 2 - 1]) as f64 / 2.0
    } else {
        return r[(n - 1) / 2] as f64
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
