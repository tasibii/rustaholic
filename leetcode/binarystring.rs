fn add_binary(a: String, b: String) -> String {
    // convert binary string to decimal num
    let a_uint = u32::from_str_radix(&a, 2).unwrap();
    let b_uint = u32::from_str_radix(&b, 2).unwrap();

    let sum = a_uint + b_uint;
    
    // format num to binary
    format!("{:b}", sum)
}

fn main() {
    let a = String::from("11");
    let b = String::from("1");
    println!("{}", add_binary(a,b));
}