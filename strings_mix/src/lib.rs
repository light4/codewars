use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    prefix: char,
    order: usize,
    c: char,
    count: usize,
}

fn mix(s1: &str, s2: &str) -> String {
    let mut result: Vec<Map> = vec![];
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    for c in s1.to_owned().chars() {
        if c.is_lowercase() {
            let counter = map1.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    for c in s2.to_owned().chars() {
        if c.is_lowercase() {
            let counter = map2.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    let mut keys = HashSet::new();
    for &c in map1.keys().chain(map2.keys()) {
        keys.insert(c);
    }
    for c in keys {
        let n1 = map1.get(&c);
        let n2 = map2.get(&c);
        let count1 = match n1 {
            Some(&n) => n,
            None => 0,
        };
        let count2 = match n2 {
            Some(&n) => n,
            None => 0,
        };
        if count1 > 1 || count2 > 1 {
            if count1 > count2 {
                let test = Map{ prefix: '1', order: 1, c: c, count: count1 };
                result.push(test);
            } else if count1 < count2 {
                let test = Map{ prefix: '2', order: 2, c: c, count: count2 };
                result.push(test);
            } else {
                let test = Map{ prefix: '=', order: 3, c: c, count: count2 };
                result.push(test);
            }
        }
    }
    result.sort_by(|a, b| b.count.cmp(&a.count).reverse()
                           .then(b.order.cmp(&a.order)).reverse()
                           .then(a.c.cmp(&b.c)));
    let mut new_result = vec![];
    for map in result {
        new_result.push(format!("{}:{}", map.prefix, map.c.to_string().repeat(map.count)));
    }
    new_result.join("/")
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn basics_mix() {

    testing("Are they here",
            "yes, they are here",
            "2:eeeee/2:yy/=:hh/=:rr");
    testing("looping is fun but dangerous",
            "less dangerous than coding",
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
    testing(" In many languages",
            " there's a pair of functions",
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing("A generation must confront the looming ",
            "codewarrs",
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");

}
