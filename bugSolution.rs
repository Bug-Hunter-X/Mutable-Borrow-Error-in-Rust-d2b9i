fn main() {
    let mut x = 5;
    { // Use a scope to restrict the lifetime of the mutable borrow
        let y = &mut x;
        *y = 6;
    }
    let z = &mut x; 
    *z = 7; 
    println!("{}, {}", x, z);
}
