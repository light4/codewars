use std::cmp::max;

fn converter(n: f64, decimals: u8, base: f64) -> String {
    let mut num = n.abs();
    let mut k = (num.ln() / base.ln()).trunc() as i32 + 1;
    if k < 0 {
        k = 0;
    }

    let mut digits = vec![];
    let mut to_add = vec![];

    let mut i = k - 1;
    loop {
        if i < -(decimals as i32) {
            break;
        }
        let digit = ((num / base.powi(i)) % base).trunc();
        num -= digit * base.powi(i);
        let c = char::from_digit(digit as u32, max(base as u32, 10))
            .unwrap()
            .to_ascii_uppercase();

        if c != '0' || digits.len() < k as usize {
            digits.extend(&to_add);
            digits.push(c);
            to_add.clear();
        } else {
            to_add.push('0');
        }

        if num <= 0.0 {
            break;
        }
        i -= 1;
    }

    if digits.len() < k as usize {
        digits.extend(['0'].repeat(k as usize - digits.len()));
    }
    let l = digits.len() - k as usize;
    if decimals as usize > l {
        digits.extend(['0'].repeat(decimals as usize - l));
    }

    if digits.len() > k as usize {
        digits.insert(k as usize, '.');
        if k == 0 {
            digits.insert(0, '0');
        }
    }

    if n.is_sign_negative() {
        digits.insert(0, '-');
    }

    digits.iter().collect()
}

fn main() {
    let test = converter(0.0, 4, 26.0);
    dbg!(&test);
}

#[cfg(test)]
mod tests {
    use super::converter;

    #[test]
    fn basic_tests_base_pi_no_decimals() {
        assert_eq!(
            converter(13.0, 0, std::f64::consts::PI),
            "103",
            "It should convert 13.0 into 103"
        );
        assert_eq!(
            converter(10.0, 0, std::f64::consts::PI),
            "100",
            "It should convert 10.0 into 100"
        );
        assert_eq!(
            converter(std::f64::consts::PI, 0, std::f64::consts::PI),
            "10",
            "It should convert pi into 10"
        );
    }
    #[test]
    fn basic_tests_base_pi() {
        assert_eq!(
            converter(13.0, 3, std::f64::consts::PI),
            "103.010",
            "It should convert 13.0 into 103.010 base pi"
        );
        assert_eq!(
            converter(10.0, 2, std::f64::consts::PI),
            "100.01",
            "It should convert 10.0 into 103 base pi"
        );
        assert_eq!(
            converter(std::f64::consts::PI, 5, std::f64::consts::PI),
            "10.00000",
            "It should convert pi into 10.0000 base pi"
        );
    }
    #[test]
    fn other_bases_no_decimals() {
        assert_eq!(
            converter(13.0, 0, 8.0),
            "15",
            "It should convert 13.0 into 15 base 8.0"
        );
        assert_eq!(
            converter(10.0, 0, 16.0),
            "A",
            "It should convert 10.0 into A base 16.0"
        );
        assert_eq!(
            converter(std::f64::consts::PI, 0, 2.0),
            "11",
            "It should convert pi into 11 base 2.0"
        );
        assert_eq!(
            converter(7.0, 0, 19.0),
            "7",
            "It should convert 7.0 into 7 base 19.0"
        );
        assert_eq!(
            converter(1.0, 0, 2.0),
            "1",
            "It should convert 1.0 into 1 base 2.0"
        );
    }
    #[test]
    fn non_integers_other_bases() {
        assert_eq!(
            converter(13.5, 4, 16.0),
            "D.8000",
            "It should convert 13.5 into D.8000 base 16.0"
        );
        assert_eq!(
            converter(10.5, 0, 16.0),
            "A",
            "It should convert 10.5 into A base 16.0"
        );
        assert_eq!(
            converter(1.0, 2, 2.0),
            "1.00",
            "It should convert 1.0 into 1.00 base 2.0"
        );
    }
    #[test]
    fn negative_and_zero() {
        assert_eq!(
            converter(-10.0, 0, 23.0),
            "-A",
            "It should convert -10.0 into -A base 23.0"
        );
        assert_eq!(
            converter(0.0, 4, 26.0),
            "0.0000",
            "It should convert 0.0 into 0.0000 base 26.0"
        );
        assert_eq!(
            converter(-15.5, 2, 23.0),
            "-F.BB",
            "It should convert -15.5 into -F.BB base 23.0"
        );
    }
    #[test]
    fn base_10() {
        assert_eq!(
            converter(13.0, 0, 10.0),
            "13",
            "It should keep 13.0 into 13 base 10.0"
        );
        assert_eq!(
            converter(10.0, 0, 10.0),
            "10",
            "It should keep 10.0 into 10 base 10.0"
        );
        assert_eq!(
            converter(5.5, 1, 10.0),
            "5.5",
            "It should keep 5.5 into 5.5 base 10.0"
        );
    }
}
