use std::fmt::{Debug, Display, Formatter, Result};

fn print_ln() {
    // print with argument
    println!("{} days", 27);
    // print multiple arguments by index
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // print multiple arguments by naming
    println!(
        "My name is {name}, {age} years old, and i'm currently based in {address}", 
        name="John", 
        age=20, 
        address="America"
    );
    // print different formatting
    println!("Base 10               :{}",   69420); // 69420
    println!("Base 2 (binary)       :{:b}", 69420); // 10000111100101100
    println!("Base 8 (octal)        :{:o}", 69420); // 207454
    println!("Base 16 (hexadecimal) :{:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal) :{:X}", 69420); // 10F2C
    // print padding number or space
    println!("Padding space from left  :{number:>5}", number=1); // _____1 (_ is space)
    println!("Padding zero from left   :{number:0>5}", number=1); // 000001
    println!("Padding zero from right  :{number:0<5}", number=1); // 100000
    // naming format specifier by adding `$`
    println!("Naming format specifier  :{number:0>width$}", number=1, width=5); // 000001
}

fn debug_print() {
    #[derive(Debug)]
    struct Person <'a> {
        name: &'a str,
        age: u8,
        address: &'a str,
    }
    let person =  Person { name: "Bob", age: 20, address: "Canada" };
    println!("Name           : {}", person.name);
    println!("Age            : {}", person.age);
    println!("Address        : {}", person.address);
    println!("Debug          : {:?}", person);
}

fn formatter_print() {
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // implement display for point2D
    impl Display for Point2D {
        fn fmt(&self, fmt: &mut Formatter) -> Result {
            write!(fmt, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point2D{x: 0.1, y: 10.3};
    // Comparisons between Debug print and Display print
    println!("Display: {}", point);
    println!("Debug:   {:?}", point);
}

fn rgb_print() {
    #[derive(Debug)]
    struct RGB {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for RGB {
        fn fmt(&self, fmt: &mut Formatter) -> Result {
            // method 1 calculate rgb via formal R * (256^2) + G * (256^1) + B * (256 ^ 0) -> format in hex 
            // let rgb = (self.red as u64) * 65536 + (self.green as u64) * 256 + self.blue as u64; 
            // write!(fmt, "RGB ({red}, {green}, {blue}) 0x{rbg:06X}", red=self.red, green=self.green, blue=self.blue, rbg=rgb)
            
            // method 2
            write!(
                fmt, 
                "RGB ({red}, {green}, {blue}) 0x{red:02X}{green:02X}{blue:02X}", 
                red=self.red, 
                green=self.green, 
                blue=self.blue
            )
        }
    }

    let rgb1 =  RGB {red: 128, green: 255, blue: 90};
    let rgb2 =  RGB {red: 0, green: 3, blue: 254};
    let rgb3 =  RGB {red: 0, green: 0, blue: 0};

    println!("{}", rgb1);
    println!("{}", rgb2);
    println!("{}", rgb3);
}

fn main() {
    print_ln();
    debug_print();
    formatter_print();
    rgb_print();
}