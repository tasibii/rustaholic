use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64, 'a', true
    );
    // print with debug
    println!("Long tuple: {:?}", long_tuple);
    // extract the values from the long tuple by index
    println!("First value: {:?}", long_tuple.0);
    println!("Last value:  {:?}", long_tuple.11);

    let nested_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // print with debug
    println!("Long tuple: {:?}", nested_tuple);
    println!("The first value of the first tuple: {:?}", nested_tuple.0.0);

    // one element tuple
    println!("One element tuple: {:?}", (1u32,));
    println!("Just an integer: {:?}", (1u32));

    // destructured tuple
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}

