use std::io;

fn main() {
    println!("Hello World!");
    println!("This is a basic Calculator!");
    println!("Please Enter your choice from 1 to 4:");
    println!("1. Addition.");
    println!("2. Subtraction.");
    println!("3. Multiplication.");
    println!("4. Division.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
    println!("You entered: {}", input);
    let choice: i8 = input.trim().parse().expect("Number format Exception at input parsing");

    println!("Please enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
    println!("You entered: {}", input);
    let first_num: i8 = input.trim().parse().expect("Number format Exception at input parsing");

    println!("Please enter the second number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
    println!("You entered: {}", input);
    let second_num: i8 = input.trim().parse().expect("Number format Exception at input parsing");

    let mut res: f32 = 0.0;

    if choice == 1 {
        res = (
            first_num + second_num) as f32;
    } else if choice == 2 {
        res = (first_num - second_num) as f32;
    } else if choice == 3 {
        res = (first_num * second_num) as f32;
    } else if choice == 4 {
        res = first_num as f32 / second_num as f32;
    } else { println!("Invalid Choice Entered!") }
    println!("The Result is {}", res);
}
