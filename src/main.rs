use std::arch::aarch64::int32x4_t;
use std::io;

enum Operations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

struct CalculationDatum {
    f_num: i32,
    s_num: i32,
    op: Operations,
}

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
    let first_num: i32 = input.trim().parse().expect("Number format Exception at input parsing");

    println!("Please enter the second number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
    println!("You entered: {}", input);
    let second_num: i32 = input.trim().parse().expect("Number format Exception at input parsing");

    let mut res: f32 = 0.0;

    if choice == 1 {
        res= calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Addition,
        });
    } else if choice == 2 {
        res= calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Subtraction,
        });
    } else if choice == 3 {
        res= calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Multiplication,
        });
    } else if choice == 4 {
        res= calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Division,
        });
    } else { println!("Invalid Choice Entered!") }
    println!("The Result is {}", res);
}

fn calculate(data: CalculationDatum) -> f32 {
    match data.op {
        Operations::Addition => { data.f_num as f32 + data.s_num as f32 }
        Operations::Subtraction => { data.f_num as f32 - data.s_num as f32 }
        Operations::Multiplication => { data.f_num as f32 * data.s_num as f32 }
        Operations::Division => { data.f_num as f32 / data.s_num as f32 }
    }
}