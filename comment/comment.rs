fn main() {
    //---------------- REGULAR COMMENTS ----------------//
    // This is an example of a line comment.

    /*
    * This is an example of a block comment.
    */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    //---------------- DOCUMENT COMMENTS ----------------//
    //https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html

    // Calling greeting method with a custom string
    greeting("Hello, world!");
    greeting("Xin chao Vietnam!");

}

/**
     * * Imported information highlighted
     * ! Deprecated method, do not use
     * ? Question: Should this method return a value?
     * TODO: Refactor this method
     * @param myGreeting greeting string
     * 
    */
fn greeting(my_greeting: &str) {
    println!("{}", my_greeting);
}

