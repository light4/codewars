use std::collections::HashSet;

fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let mut base_str = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    for input_set in input_sets {
        for i in input_set {
            if i != '_' {
                base_str = base_str.replace(i, "_");
            }
        }
    }

    base_str
}

#[test]
fn basic_test1() {
    let mut input_set: Vec<HashSet<char>> = Vec::new();
    input_set.push(['A', 'b'].iter().cloned().collect());
    input_set.push(['C', 'd'].iter().cloned().collect());
    assert_eq!(destroy(input_set), "a _ c _ e f g h i j k l m n o p q r s t u v w x y z");
}

#[test]
fn basic_test2() {
    let mut input_set: Vec<HashSet<char>> = Vec::new();
    input_set.push(['B', 'b'].iter().cloned().collect());
    input_set.push(['C', 'm', 'f'].iter().cloned().collect());
    assert_eq!(destroy(input_set), "a _ c d e _ g h i j k l _ n o p q r s t u v w x y z");
}
