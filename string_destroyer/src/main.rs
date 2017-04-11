use std::collections::HashSet;

fn main() {
    let mut base_str = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    let mut input_set: Vec<HashSet<char>> = Vec::new();
    input_set.push(['B', 'b'].iter().cloned().collect());
    input_set.push(['C', 'm', 'f'].iter().cloned().collect());
    for input_array in input_set {
        for i in input_array {
            base_str = base_str.replace(i, "_");
        }
    }

    println!("base: {:?}", base_str);
}
