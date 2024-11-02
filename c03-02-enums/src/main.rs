/*
 * `enum` allows the creation of a type which may be one of a few different variants.
 * Any variant which is valid as a `struct` is also valid in an `enum`
 */
enum WebEvent {
    /*
     * An `enum` can be
     * 1. like a unit struct
     * 2. like a tuple struct
     * 3. like a c-like struct
     */
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

/*
 * You can use a type alias to refer an enum that is too long or too generic
 */
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn inspect(event: WebEvent) {
    /*
     * You can pattern match an enum and destructure its values accordingly
     */
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted: {}", s),
        WebEvent::Click { x, y } => println!("Clicked: ({}, {})", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 20 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
