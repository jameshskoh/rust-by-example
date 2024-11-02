fn main() {
    /*
     * Rust checks formatting correctness at compile time
     *
     * Common printing macros
     *
     * format!      format text
     * print!       formatted text is printed to console (io::stdout)
     * println!     same as print!, but appends a new line
     * eprint!      same as print!, but prints to standard error io::stderr
     * eprintln!:   same as eprint!, but appends a new line
     */
    println!("Hello, world!");

    /*
     * {} will be automatically replaced with arguments
     * 1. The arguments will be stringified
     * 2. The arguments have to implement fmt::Display to support formatting
     */
    println!("{} days", 31);

    /*
     * Positional arguments are supported using integers
     */
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    /*
     * Named arguments are supported
     */
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    /*
     * You can format integers to different bases
     */
    println!("Base 10:  {}", 69420);
    println!("Base 2:   {:b}", 69420);
    println!("Base 8:   {:o}", 69420);
    println!("Base 16:  {:x}", 69420);

    /*
     * You can right justify text, pad numbers with extra zeroes
     */
    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);

    /*
     * You can parameterize format specifiers by using variables appended with `$`
     */
    println!("{number:0>width$}", number = 1, width = 5);

    /*
     * You can capture arguments from a surrounding variable
     */
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:width$}");
}
