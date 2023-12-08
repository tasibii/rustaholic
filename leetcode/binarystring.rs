fn add_binary(a: String, b: String) -> String {
    // overflow bugs ||
    //               v
    // //convert binary string to decimal num
    // //let a_uint = usize::from_str_radix(&a, 2).unwrap();
    // //let b_uint = usize::from_str_radix(&b, 2).unwrap();

    // //let sum = a_uint + b_uint;
    // //format!("{:b}", sum)

    // new solution
    // idea is loop from last to first each iter and sum convert to bin string
    let (mut carry, mut result, mut rev_a, mut rev_b) = (0, "".to_string(), a.chars().rev(), b.chars().rev());
    loop {
        let iter_sum = match (rev_a.next(), rev_b.next()) {
            (None, None) => break,
            (Some(x), None) | (None, Some(x)) => x.to_digit(2).unwrap(),
            (Some(_a), Some(_b)) => _a.to_digit(2).unwrap() + _b.to_digit(2).unwrap(),
        };
        result.push(char::from_u32('0' as u32 + (iter_sum + carry) % 2).unwrap());
        carry = (iter_sum + carry) / 2;
    }

    if carry > 0 {
        result.push('1');
    }
    result.chars().rev().collect()
}

fn main() {
    let a = String::from("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
    let b = String::from("1");
    println!("{}", add_binary(a, b));
}