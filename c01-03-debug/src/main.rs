/*
 * Newly created types do not support formatting, you need to implement it manually.
 */
struct UnPrintable(i32);

/*
 * There is an easy way for a type to support formatting through fmt::Debug, by adding #[derive(Debug)] attribute
 */
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
// No idea what this magical syntax is ...
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    /*
     * {} does not support fmt::Debug, for fmt::Debug objects, you have to use {:?}
     * You have no control over what is displayed using #[derive(Debug)]
     */
    println!("{:?}", 12);
    println!("{:?}", Structure(3));
    println!("{:?}", Deep(Structure(7)));
    println!("Debug print: {:?}", Person { name: "a", age: 27 });

    /*
     * Rust supports pretty printing using {:#?} for fmt::Debug
     */
    println!("Pretty print: {:#?}", Person { name: "a", age: 27 } );
}
