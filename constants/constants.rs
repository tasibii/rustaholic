// In Rust, both static and const are used for defining constants, but they have some key differences.
// 1. Mutability: 
// a. const: is always immutable
// b. static: can be mutable (static mut <varName>)
//
// 2. Lifetime:
// a. const: is limited to the scope in which it is defined.
// b. static: is potentially for the entire duration of the program.
//
// 3. Initialization:
// a. const: must be initialized with a constant expression, which is evaluated at compile time.
// b. static: can be initialized with both constant and non-constant expressions. If it's initialized 
// with a non-constant expression, it will be evaluated at runtime.

static COUNTRY: &str = "Vietnam";
static mut CITY: &str = "HoChiMinh";

const DENOMINATOR: f32 = 10_000.0;
fn main() {
    println!("This is {}", COUNTRY);
    println!("The denominator is {}", DENOMINATOR);
    let share = 3000.0;
    println!("Percentage: {}", (share / DENOMINATOR) as f32);

    unsafe {
        CITY = "HaNoi";
        println!("{}", CITY);
    }
}