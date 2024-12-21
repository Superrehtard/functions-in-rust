fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(5, 'h');

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1

    // x + 1; // This will cause an error because the function is expected to return a value
    // statements do not return values and end with a semicolon
    // expressions evaluate to a resulting value and do not end with a semicolon
}
