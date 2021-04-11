#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    buttom_right: Point,
}

pub fn main_truct() {
    // Create sturc with field init shorthand
    let name = String::from("Ivan");
    let age = 27;
    let ivan = Person { name, age };

    // Print debug struct
    println!("{:?}", ivan);

    // Instantiate a 'Point'
    let point: Point = Point { x: 10.3, y: 0.4 };
    // Access the fields of the point
    println!("Point cordinates ({}, {})", point.x, point.y);

    // Make a new point by using struc update syntax to use the fields of our other one
    let buttom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", buttom_right.x, buttom_right.y);

    // Destructure the point using a 'let binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        buttom_right: buttom_right,
    };

    // Instantiate a unite struct
    let _unit = Unit;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destruture a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
