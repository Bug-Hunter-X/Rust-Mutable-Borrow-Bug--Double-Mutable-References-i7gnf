fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y = 6; // Modify x through y
    *z = 7; // Modify x through z

    println!("x: {}", x); // This will print 7
}
This code has a subtle bug in Rust related to mutable borrows. While it compiles, it's considered bad practice because of aliasing, potentially leading to data races or unexpected behavior if not handled carefully.