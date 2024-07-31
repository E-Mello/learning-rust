pub fn control_flow() {
    // Rust has three kinds of control flow statements: if expressions, loops, and match expressions
    // If expressions allow you to branch based on conditions
    let number = 3;
    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    // If expressions can be used in let statements
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Loops allow you to repeat code until a condition is met
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // While loops allow you to repeat code while a condition is true
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");

    // For loops allow you to iterate over a collection
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // For loops can also be used with ranges
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!");
}
