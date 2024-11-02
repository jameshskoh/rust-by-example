/*
 * Types of structures
 * 1. Tuple structs, basically named tuples
 * 2. Classic C structs
 * 3. Unit structs, field-less, useful for generics
 */

struct Person {
    name: String,
    age: u8,
}

/*
 * A unit struct
 */
struct Unit;

/*
 * A tuple struct
 */
struct Pair(i32, f32);

/*
 * A struct with 2 fields
 */
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

/*
 * Struct can be used as a field of another struct
 */
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: top_left_x,
            y: top_left_y
        },
        bottom_right: Point {
            x: bottom_right_x,
            y: bottom_right_y
        }
    } = rect;

    let height = bottom_right_y - top_left_y;
    let width = bottom_right_x - top_left_x;
    width * height
}

fn square(top_left: Point, length: f32) -> Rectangle {
    let Point { x: top_left_x, y: top_left_y } = top_left;
    Rectangle { top_left, bottom_right: Point { x: top_left_x + length, y: top_left_y + length } }
}

fn main() {
    /*
     * Instantiate a unit struct, nothing special
     */
    let a_unit = Unit;

    /*
     * Instantiate a tuple struct
     */
    let pair = Pair(1, 0.1);

    /*
     * Accessing fields of a tuple struct
     */
    println!("The pair are {} and {}", pair.0, pair.1);

    /*
     * Destructure a tuple struct
     */
    let Pair(an_integer, a_decimal) = pair;

    /*
     * Create a classic C struct
     *
     * Question: Difference between String and &str?
     */
    let sam = Person { name: String::from("Sam"), age: 10 };

    /*
     * Create a classic C struct with shorthand
     */
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    /*
     * Access the fields of a Person
     */
    println!("This is {}, he is {} years old", peter.name, peter.age);

    /*
     * You can create another Person by copying some fields from a Person
     * Jenny is same years old as Peter
     */
    let jenny = Person { name: String::from("Jenny"), ..peter };

    /*
     * You can destructure a struct
     */
    let Person { name: jenny_name, age: jenny_age } = jenny;
    println!("This is {}, she is {} years old", jenny_name, jenny_age);

    /*
     * Instantiate a struct that contains other structs?
     * It is possible, as struct instantiation is an expression too
     */
    let a_rectangle = Rectangle {
        top_left: Point { x: 1.0, y: 2.0 },
        bottom_right: Point { x: 10.0, y: 20.0 },
    };

    println!("The area of the rectangle is {}", rect_area(&a_rectangle));

    println!("A square is created: {:#?}", square(Point { x: 2.0, y: 3.0 }, 5.0));
}
