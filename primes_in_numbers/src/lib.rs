use std::collections::BTreeMap;

fn prime_factors(n: i64) -> String {
    let mut result = BTreeMap::new();
    let mut prime = 2;
    let mut test = n;
    while prime <= test {
        if test % prime == 0 {
            test /= prime;
            let counter = result.entry(prime).or_insert(0);
            *counter += 1;
        } else {
            prime += 1;
        }
    }
    let mut output = String::from("");
    for (key, val) in result.iter() {
        if *val != 1 {
            output.push_str(format!("({}**{})", key, val).as_str());
        } else {
            output.push_str(format!("({})", key).as_str());
        }
    }
    output
}

fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17*17*93*677, "(3)(17**2)(31)(677)");
    
}
