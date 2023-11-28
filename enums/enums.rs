enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

// enum with implicit (default starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit 
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum Math {
    Add,
    Sub,
    Mul,
    Div,
}

impl Math {
    fn run(&self, x: i64, y: i64) -> i64 {
        match self {
            Math::Add => x + y,
            Math::Sub => x - y,
            Math::Mul => x * y,
            Math::Div => x / y,
        }
    }
}

// type aliases for math enum 
type Operations = Math;

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Color::{Red, Green, Blue};
    use crate::Number::{Zero, One, Two};
    use crate::WebEvent::{PageLoad, PageUnload, KeyPress, Paste, Click};

    inspect(PageLoad);
    inspect(PageUnload);
    inspect(KeyPress('x'));
    inspect(Paste("Hello, world!".to_owned()));
    inspect(Click {x: 10, y: 10});

    println!("zero is: {}", Zero as i32);
    println!("one is:  {}", One as i32);
    println!("two is:  {}", Two as i32);

    println!("red hex:   #{:06x}", Red as i32);
    println!("green hex: #{:06x}", Green as i32);
    println!("blue hex:  #{:06x}", Blue as i32);

    println!("add result: {}", Operations::Add.run(4,2));
    println!("sub result: {}", Operations::Sub.run(4,2));
    println!("mul result: {}", Operations::Mul.run(4,2));
    println!("div result: {}", Operations::Div.run(4,2));
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted '{}'", s),
        WebEvent::Click {x, y} => println!("click at coordinates ({}, {})", x, y),
    }
}