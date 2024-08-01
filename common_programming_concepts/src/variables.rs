pub fn variables() {
    // Variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);

    // Variables can be made mutable by using the mut keyword
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // Constants are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing allows you to change the type of a variable
    let z = 5;
    println!("The value of z is: {}", z);
    let z = z + 1;
    println!("The value of z is: {}", z);
    let z = z * 2;
    println!("The value of z is: {}", z);

    // Shadowing also allows you to change the mutability of a variable
    let a = 5;
    println!("The value of a is: {}", a);
    let a = a + 1;
    println!("The value of a is: {}", a);
    let mut a = a;
    a = a * 2;
    println!("The value of a is: {}", a);
}
