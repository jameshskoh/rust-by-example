/*
 * Functions can use tuples to return multiple values, as tuples can hold any number of values
 */
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    /*
     * destructure a tuple
     */
    let (int_param, bool_param) = pair;

    /*
     * construct a tuple
     */
    (bool_param, int_param)
}
fn main() {
    let long_tuple = (1u8, 2u16, 3u32, -1i8, 0.1f32, 'a', true);

    /*
     * Tuple values can be extracted by their indexes
     */
    println!("First element: {}", long_tuple.0);

    /*
     * A tuple can hold another tuple
     */
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (-1i8, -2i16, -3i32));

    /*
     * Tuples (up to 12 elements) are printable via fmt::Debug
     *
     */
    println!("Tuple of tuples: {:?}", tuple_of_tuples);

    /*
     * Applying a function on a tuple
     */
    println!("A reversed pair: {:?}", reverse((1, true)));

    /*
     * To create a 1-tuple, you need a comma to tell it apart from a literal that is surrounded by parentheses
     */
    let one_tuple = (1,);
    let literal = (1); // unnecessary parentheses ...
}
