fn main() {
    let mut x = 5;
    // Instead of creating multiple mutable references,  borrow x once and mutate accordingly
    {
        let y = &mut x; 
        *y = 6; 
    }
    {
        let z = &mut x; 
        *z = 7;
    }
    println!("x: {}", x);
}

//Alternative solution using cloning
fn main() {
    let mut x = 5;
    let mut y = x.clone(); // Create a copy
    let mut z = x.clone(); // Create a copy
    y = 6; //Modify the copy
    z = 7; //Modify the copy
    println!("x: {}", x); // x remains unchanged
    println!("y: {}", y); //y shows a modified value
    println!("z: {}", z); //z shows a modified value
} 