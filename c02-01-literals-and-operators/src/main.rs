fn main() {
    /*
     * Primitive values can be expressed using literals
     *
     * Numeral literals:
     * 1. Integers can be expressed using hexadecimal, octal, or binary notation
     */

    /*
     * This example shows why suffix notation is important, and also the difference between i32 and u32
     */
    println!("1 + 2 = {}", 1u32 + 2);

    // compiler would complain about this, as overflow is detected
    // println!("1 - 2 = {}", 1u32 - 2);

    /*
     * Scientific notation is supported
     */
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    /*
     * Underscores can be inserted in numeric literals to improve readability
     */
    println!("One million: {}", 1_000_000u32);

    /*
     * Boolean logic        &&, ||, and !
     * Bitwise operations   &, |, ^ (XOR), <<, and >>
     */
    println!("0011 AND 0101 is 0b{:04b}", 0b0011 & 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x0080 >> 2 is 0x{:04x}", 0x80u32 >> 2);
}
