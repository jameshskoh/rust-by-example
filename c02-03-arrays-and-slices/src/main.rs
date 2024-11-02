/*
 * An array is a collection of objects of the same type T, stored in contiguous memory.
 * An array's length is known at compile time
 *
 * Slices are similar to arrays, but their length is not known at compile time
 * A slice is a two-word object, the first is a pointer to the data, the second is the length of the slice
 * Slices can be used to borrow a section of an array, and have the type signature &[T]
 */
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
   let xs: [i32; 5] = [1, 2, 3, 4, 5];

    /*
     * All elements can be initialized to the same value
     */
    let ys: [i32; 500] = [1; 500];

    /*
     * Indexing starts from 0
     */
    println!("First element is {}", xs[0]);

    /*
     * `len()` returns the number of elements in the array
     */
    println!("Number of elements is {}", xs.len());

    /*
     * Arrays are stack allocated
     */
    println!("The array occupies {} bytes", mem::size_of_val(&xs));

    /*
     * Arrays can be borrowed as slices
     * Slices can be borrowed as a whole, or as a "slice"
     */
    analyze_slice(&xs);
    analyze_slice(&xs[1..4]);

    /*
     * Empty array and empty slices
     */
    let empty_array: [i32; 0] = [];

    /*
     Empty slices are always equal
     */
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    /*
     * Arrays can be accessed safely using `.get()`, which returns an `Option`
     * Compared to OOB indexing, which is a compile error
     */
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            // matches a Some Option
            Some(xval) => println!("{}: {}", i, xval),
            // matches a None Option
            None => println!("Slow down! {} is too far!", i),
        }
    }

    /*
     * This won't compile, as an array's size is known at compile time
     */
    // println!("{}", xs[5])

    /*
     * This will compile, as a slice's size is not known at compile time
     * This will throw an error at run time
     */
    println!("{}", xs[..][5]);
}
