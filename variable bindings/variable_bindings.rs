fn main() {
    mutability();
    block_scope();
    freezing();
}

fn mutability() {
    let immutable_variable = 1;
    let mut mutable_variable = 1;

    println!("Immutable variable: {}", immutable_variable);
    println!("Before mutation: {}", mutable_variable);
    mutable_variable += 1;
    println!("After mutation: {}", mutable_variable);

    // error[E0384]: cannot assign twice to immutable variable `immutable_variable`
    // immutable_variable += 1;
}

fn block_scope() {
    let long_live_variable = 1;
    let shadowed_variable = 999;

    {   
        let short_live_variable = 5;
        println!("Inner block variable: {}", short_live_variable);
        println!("Before being shadowed in the inner block: {}", shadowed_variable);
        let shadowed_variable = "abc";
        println!("After being shadowed in the inner block: {}", shadowed_variable);
    }

    println!("The outer block variable: {}", long_live_variable);

    // error[E0425]: cannot find value `short_live_variable` in this scope
    // println!("The outer block retrieves the inner block variable: {}", short_live_variable);

    println!("The outer block retrieves the inner block shadowed: {}", shadowed_variable);
    let shadowed_binding = 2;
    println!("After being shadowed in the outer block: {}", shadowed_binding);
}

fn freezing() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // error[E0384]: cannot assign twice to immutable variable `_mutable_integer`
        // _mutable_integer = 50;
    }
    // `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}