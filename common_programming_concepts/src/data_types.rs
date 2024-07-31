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

    // Booleans can be true or false
    let boolean: bool = true;
    println!("The value of boolean is: {}", boolean);
    let boolean: bool = false;
    println!(" The value of boolean is: {}", boolean);

    // Characters are specified with single quotes
    let character: char = 'z';
    println!("The value of character is: {}", character);
}
