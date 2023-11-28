use std::fmt::Debug;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Unit;

#[derive(Debug)]
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let unit = Unit;
    let pair = Pair(2i32, 2.5f32);
    let point = Point {x: 10.5, y: 20.1};
    let person = Person {name: String::from("Alice"), age:18};
    let rect = Rectangle {top_left: Point{x: 0.0, y:10.1}, bottom_right: Point{x:1.5, y: 2.5}};
    
    println!("Unit: {:?}", unit);
    println!("Pair:          {:?}", pair);
    println!("Pair contains: {:?} & {:?}", pair.0, pair.1);

    println!("Point:             {:?}", point);
    println!("Point coordinates: ({}, {}) ", point.x, point.y);

    println!("Person: {:?}", person);
    println!("Name:   {:?}", person.name);
    println!("Age:    {:?}", person.age);

    println!("Rectangle: {:?}", rect);
    println!("Rectangle top-left coordinates:     ({}, {})", rect.top_left.x, rect.top_left.y);
    println!("Rectangle bottom_right coordinates: ({}, {})", rect.bottom_right.x, rect.bottom_right.y);
    println!("Rectangle area:                     {}", rect_area(rect));

    let square: Rectangle = square(point, 3.0);
    println!("Square: {:?}", square);
    println!("Square area: {:?}", rect_area(square));
}

fn rect_area(rect: Rectangle) -> f32 {
    let top_left = rect.top_left;
    let bottom_right = rect.bottom_right;
    (bottom_right.x - top_left.x).abs() * (bottom_right.y - top_left.y).abs()
}

fn square(top_left: Point, side: f32) -> Rectangle {
    let bottom_right: Point = Point {x: top_left.x + side, y: top_left.y + side};
    Rectangle {top_left, bottom_right}
}
