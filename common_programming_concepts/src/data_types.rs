pub fn data_types() {
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time
    // The compiler can usually infer the type of a variable based on the value assigned to it
    let guess = "42";
    println!("The value of guess is: {}", guess);

    // If the compiler can't infer the type, you can explicitly specify it
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // Scalar types represent a single value
    // Integers can be signed or unsigned and have different sizes
    let integer: i8 = 127;
    println!("The value of integer is: {}", integer);
    let integer: u8 = 255;
    println!("The value of integer is: {}", integer);
    let integer: i16 = 32767;
    println!("The value of integer is: {}", integer);
    let integer: u16 = 65535;
    println!("The value of integer is: {}", integer);
    let integer: i32 = 2147483647;
    println!("The value of integer is: {}", integer);
    let integer: u32 = 4294967295;
    println!("The value of integer is: {}", integer);
    let integer: i64 = 9223372036854775807;
    println!("The value of integer is: {}", integer);
    let integer: u64 = 18446744073709551615;
    println!("The value of integer is: {}", integer);
    let integer: i128 = 170141183460469231731687303715884105727;
    println!("The value of integer is: {}", integer);
    let integer: u128 = 340282366920938463463374607431768211455;
    println!("The value of integer is: {}", integer);

    // Floating-point numbers can be f32 or f64
    let float: f32 = 3.0;
    println!("The value of float is: {}", float);
    let float: f64 = 3.0;
    println!("The value of float is: {}", float);

    // Numeric operations
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);
    let product = 4 * 30;
    println!("The value of product is: {}", product);
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    // Booleans can be true or false
    let boolean: bool = true;
    println!("The value of boolean is: {}", boolean);
    let boolean: bool = false;
    println!(" The value of boolean is: {}", boolean);

    // Characters are specified with single quotes
    let character: char = 'z';
    println!("The value of character is: {}", character);

    // Compound types can group multiple values into one type
    // Tuples can hold multiple values of different types
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: {:?}", tuple);
    let (x, y, z) = tuple;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of tuple.0 is: {}", tuple.0);
    println!("The value of tuple.1 is: {}", tuple.1);
    println!("The value of tuple.2 is: {}", tuple.2);

    // Arrays have a fixed length and all elements must have the same type
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", array);
    let first = array[0];
    println!("The value of first is: {}", first);
    let second = array[1];
    println!("The value of second is: {}", second);

    // Invalid array element access
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    let index: usize = index.trim().parse().expect("Index must be a number.");
    let element = a[index];
    println!("The value of  element is: {}", element);

    // This will cause a panic
    // let element = array[index];
    // println!("The value of element is: {}", element);
}
