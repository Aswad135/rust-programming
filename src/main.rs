use std::io;

enum Operations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Mean,
    Remainder,
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
    println!("5. Mean.");
    println!("6. Remainder.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
    println!("You entered: {}", input);
    let choice: i8 = input.trim().parse().expect("Number format Exception at input parsing");


    let mut res: f32 = 0.0;
    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;
    if choice == 5 {
        println!("Please enter the numbers separated by comma.:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
        println!("You entered: {}", input);
        let numbers = input.trim().split(",");
        let mut vec_num: Vec<i32> = vec![];
        for num in numbers {
            let mut num_clone = num.clone();
            vec_num.push(num_clone.parse().unwrap());
        }
        res = find_mean(vec_num);
    } else {
        println!("Please enter the first number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
        println!("You entered: {}", input);
        first_num = input.trim().parse().expect("Number format Exception at input parsing");

        println!("Please enter the second number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to fetch user input!");
        println!("You entered: {}", input);
        second_num = input.trim().parse().expect("Number format Exception at input parsing");
    }
    if choice == 1 {
        res = calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Addition,
        });
    } else if choice == 2 {
        res = calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Subtraction,
        });
    } else if choice == 3 {
        res = calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Multiplication,
        });
    } else if choice == 4 {
        res = calculate(CalculationDatum {
            f_num: first_num,
            s_num: second_num,
            op: Operations::Division,
        });
    } else if choice == 6 {
        let tuple_demo: (i64, f64) = (first_num as i64, second_num as f64);
        res = find_remainder(tuple_demo);
    } else {
        println!("Invalid Choice Entered!")
    }
    println!("The Result is {}", res);
}

fn calculate(data: CalculationDatum) -> f32 {
    match data.op {
        Operations::Addition => { data.f_num as f32 + data.s_num as f32 }
        Operations::Subtraction => { data.f_num as f32 - data.s_num as f32 }
        Operations::Multiplication => { data.f_num as f32 * data.s_num as f32 }
        Operations::Division => { data.f_num as f32 / data.s_num as f32 }
        Operations::Mean => { 0.0 }
        Operations::Remainder => { data.f_num % data.s_num }
    }
}

fn find_mean(val: Vec<i32>) -> f32 {
    let mut sum = 0;
    for num in val {
        sum = sum + num;
    }
    (sum / val.len()) as f32
}

fn find_remainder(data: (i64, f64)) -> f32 {
    data.0 % data.1 as f32
}