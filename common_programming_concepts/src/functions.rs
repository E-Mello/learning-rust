pub fn functions() {
    // Functions are declared with the fn keyword
    fn main() {
        println!("Hello, world!");
    }
    main();

    // Functions can take parameters
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    greet("Alice");

    // Functions can return values
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    let sum = add(5, 3);
    println!("The sum is: {}", sum);

    // Functions can return multiple values
    fn div_rem(x: i32, y: i32) -> (i32, i32) {
        (x / y, x % y)
    }
    let (quotient, remainder) = div_rem(10, 3);
    println!("The quotient is: {}", quotient);
    println!("The remainder is: {}", remainder);

    // Functions can be used as arguments to other functions
    fn apply(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
        f(x, y)
    }
    let sum = apply(add, 5, 3);
    println!("The sum is: {}", sum);

    // Functions can be assigned to variables
    let multiply = |x, y| x * y;
    let product = apply(multiply, 5, 3);
    println!("The product is: {}", product);

    // Functions can be closures
    let subtract = |x, y| x - y;
    let difference = apply(subtract, 5, 3);
    println!("The difference is: {}", difference);

    // Functions can be methods
    struct Circle {
        radius: f64,
    }
    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    let circle = Circle { radius: 1.0 };
    println!("The area of the circle is: {}", circle.area());
}
