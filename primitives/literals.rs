fn main() {
    println!("\n*** integer calculation ***");
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1 * 2 = {}", 1u32 * 2);
    println!("1 / 2 = {}", 1u32 / 2);
    println!("1 % 2 = {}", 1u32 % 2);


    println!("\n*** float calculation ***");
    println!("1.1 + 2.9 = {}", 1.1f32 + 2.9f32);
    println!("1.1 - 2.9 = {}", 1.1f32 - 2.9f32);
    println!("1.1 * 2.9 = {}", 1.1f32 * 2.9f32);
    println!("1.1 / 2.9 = {}", 1.1f32 / 2.9f32);
    println!("1.1 % 2.9 = {}", 1.1f32 % 2.9f32);

    
    println!("\n*** scientific notation ***");
    println!("1e4 is:     {}", 1e4);
    println!("-2.5e-3 is: {}", -2.5e-3);

    println!("\n*** short-circuiting boolean logic ***");
    println!("true AND false is: {}", true && false);
    println!("true OR false is:  {}", true || false);
    println!("NOT true is:       {}", !true);

    println!("\n*** bitwise operations ***");
    println!("0011 AND 0101 is: {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is:  {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is: {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is:        {}", 1u32 << 5);
    println!("0x80 >> 2 is:     0x{:x}", 0x80u32 >> 2);

    println!("\n*** use underscores to improve readability ***");
    println!("One million is written as {}", 1_000_000u32);
}