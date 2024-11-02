fn main() {
    /*
     * Primitive provided:
     * Signed integers      i8, i16, i32, i64, i128, and isize
     * Unsigned integers    u8, u16, u32, u64, u128, and usize
     * Floating point       f32, f64
     * char                 Unicode characters, 4 bytes each
     * bool
     * unit type            () an empty tuple (that is not a compound type), like `never` in TypeScript(?)
     */

    /*
     * Variables can be type annotated
     */
    let logical: bool = true;

    /*
     * Number suffix annotation
     */
    let an_integer = 5i32;
    let a_float = 1.0f64;

    /*
     * Variable type can be inferred
     */
    let some_character = 'a';

    /*
     * Inferred type defaults:
     * Integer      i32
     * Float        f64
     */
    let some_float = 1.0;
    let some_integer = 1;

    /*
     * Type can be inferred from context (inference is smart)
     * big_integer is inferred as i64, because at some point it is assigned an i64 value
     *
     * Note that Rust variables are immutable by default, and you need `mut` to define a mutable variable
     */
    let mut big_integer = 10;
    big_integer = 4294967296i64;

    /*
     * Variable cannot change type, but can be overwritten by shadowing
     */
    let mut mutable = 10;
    // mutable = true; // won't work
    let mutable = true; // this works (shadowing)

    /*
     * Array is typed as [T; length]
     * The compiler will check whether the assigned array length equals the type defined
     */
    let my_array: [i32; 5] = [1, 1, 1, 1, 1];

    /*
     * Tuple is a collection of values of different types
     */
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}
