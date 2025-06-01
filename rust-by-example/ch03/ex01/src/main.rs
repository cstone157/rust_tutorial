// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: _top_left, bottom_right: _bottom_right } = rect;
    let Point { x: _x1, y: _y1 } = _top_left;
    let Point { x: _x2, y: _y2 } = _bottom_right;

    (_x2 - _x1).abs() * (_y2 - _y1).abs()
}

fn square(pnt: Point, length: f32) -> Rectangle {
    let pnt2 = Point { x: pnt.x + length, y: pnt.y + length };
    Rectangle{ top_left: pnt, bottom_right: pnt2 }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 10.3, ..another_point };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Rectangle defined by ({}, {}) and ({}, {})", 
        _rectangle.top_left.x, _rectangle.top_left.y, 
        _rectangle.bottom_right.x, _rectangle.bottom_right.y, 
    );
    println!("area of Rectangle {}", 
        rect_area(_rectangle), 
    );

    let point1 = Point{ x: 0.0, y: 0.0 };
    let rect1 = square(point1, 3.1);
    println!("Rectangle defined by ({}, {}) and ({}, {})", 
        rect1.top_left.x, rect1.top_left.y, 
        rect1.bottom_right.x, rect1.bottom_right.y, 
    );

}