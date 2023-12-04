
fn main() {
    loop_();
    for_loop();
    while_loop();
}   

fn loop_() {
    let mut counter = 0;

    println!("\nLoop infinity example...");
    println!("--------------------------");

    loop {
        counter += 1;

        if counter == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }

        println!("{}", counter);

        if counter == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
    println!("##########################");
}

fn for_loop() {

    println!("\nFor loop example...");
    println!("--------------------------");

    for n in 1..=20 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    println!("##########################");
}

fn while_loop() {
    let mut n = 1;

    println!("\nWhile loop example...");
    println!("--------------------------");

    // Loop while `n` is less than 101
    while n < 21 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
    println!("##########################");
}