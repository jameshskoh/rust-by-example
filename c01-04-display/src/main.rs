use std::fmt;

/*
 * This is a 2-tuple struct
 */
struct Structure(i32, i32);

/*
 * Apply fmt::Display trait to Structure
 */
impl fmt::Display for Structure {

    /*
     * fmt::Display trait requires this `fmt` function
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*
         * write! writes the format to the supplied output stream `f`
         * write! returns a fmt::Result which indicates whether the operation succeeded or failed.
         *
         */
        write!(f, "This is a Structure with values {} and {})", self.0, self.1)
        /*
         * Note that the above statement does not end with a semicolon
         *
         * 1. You put semicolon to suppress the result of an expression
         * 2. If you don't put a semicolon, the result of the expression is returned by the function
         * 3. You can also return an expression by writing `return expression;`, but it is not necessary
         *
         * What is an expression? An expression can be evaluated to a value
         * E.g. assuming value1 and value2 are i32: `value1 * value2`
         *
         * What is not an expression? E.g. an assignment, etc.
         * E.g. `let inner = 2;` is an assignment
         */
    }
}

#[derive(Debug)]
struct Point2D {
    x: i64, y: i64
}

/*
 * This implements a struct where fields are named
 */
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x is {}, y is {}", self.x, self.y)
    }
}

/*
 * There might be some issues implementing fmt::Display for generic structs
 * Reference: https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
 */

fn main() {
    println!("A Structure: {}", Structure(30, 50));

    let point2d = Point2D{ x: 10, y: 20 };
    println!("A Point2D: {}", point2d);
    println!("A Point2D printed using fmt::Debug: {:?}", point2d);

}
