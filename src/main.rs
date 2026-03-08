/*fn main() {
    let age = 30;
    if age >= 18{
        println!("You are an adult");
    }else{
        println!("You are a minor");
    }
}*/

//*use std::io;
/*fn main() {
let mut input =String::new();
println!("Enter a number to check if it is even or odd");

io::stdin()
.read_line(&mut input)
.expect("Failed to read line");

let number: i32 = input.trim().parse().expect("Enter a valid number");

if number % 2 == 0{
    println!(" The number is even");
}else {print!("The number is odd");


}
}*/

/*fn main(){
        loop {
            let mut input  = String::new();
            println!("Enter your name or 'exit' to stop");

io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim()== "exit"{
            break;

        }

        println!("Your name is {}", input.trim());
        }
    }*/

/*fnmai
    let mut number = 1;
    while number <= 5 {
        println!("Number is {}", number);
        number += 1;
    }n() {
}/ */

/*fn main(){

for num in 1..=20 {
if num % 2 == 0{
    println!("{}", num)}}

}*/

/*fn main(){
println!("******");
           println!("*     *");
          println! ("*     *");
          println! ("* RUST*");
          println! ("*     *");
          println! ("******") ;

       }*/

/*fn main(){
    let mut age = 30;
    let mut height = 1.6;
    let is_girl: bool= true;
    let grade: char = 'Y';

    println!("I am {} years old.", age);
    println!("My height is {} .", height);
    println!("It is {}.", is_girl);
    println!("and {}.", grade);
    println!("I am {} years old. My height is {} . It is {} . and {} .", age , height, is_girl , grade);

}*/

/*use std::io;

// fn main() {
    let num = 100;
    if num >= 80 {
        println!("A")
    } else if num >= 70 {
        println!("B")
    } else if num >= 60 {
        println!("C")
    } else if num >= 50 {
        println!("D")
    } else if num < 50 {
        println!("F")
    } else {
        println!("Error")
    }
 }*/

/*use std::io;

fn main() {
    let mut name: String = String::new();
    let mut age = String::new();

    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter your age: ");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    println!("Hello,{} you are {} years old", name.trim(), age.trim());

    let age: u32 = age.trim().parse().expect("Ivalid age");
    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are a minor");
    }
}*/

/*fn main() {
    let my_scores = [100, 90, 80, 70, 60];
    println!("My scores are: {:?}", my_scores);
    println!("The first score is:{}", my_scores[0]);
}*/

/*fn main() {
    let age = 10;

    let mut number: Option<i32> = Some(30);
    match number {
        Some(n) => println!("Number is {}", n),
        None => println!("Number is not present"),
    }

    number = None;
    match number {
        Some(n) => println!("Number is {}", n),
        None => println!("Number is not present"),
    }
}*/

/*fn main() {
    let mut number: Option<&str> = None;
    number = Some("John");
    match number {
        Some(n) => println!("Number is  {}", n),
        None => println!("Number is not present"),
    }

    let num1 = 40;
    let num2 = 20;
    let result = divide(num1, num2);
    match result {
        Some(n) => println!("Result of {} / {} is {}", num1, num2, n),
        None => println!("Division by zero is not allowed"),
    }

    let result = division(num1, num2);
    match result {
        Ok(n) => println!("Result of {} / {} is {}", num1, num2, n),
        Err(e) => println!("{}", e),
    }
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}/ */

/*fn main() {
    let cash = PaymentMethod::Cash;
    let card = PaymentMethod::Card;
    let transfer = PaymentMethod::Transfer;

    paid(card)
}
enum PaymentMethod {
    Cash,
    Card,
    Transfer,
}

fn paid(pm: PaymentMethod) {
    match pm {
        PaymentMethod::Card => println!("paid cash"),
        PaymentMethod::Cash => println!("Paid with card"),
        PaymentMethod::Transfer => println!("Bank transfer used"),
    }
}*/

/*use std::fs::File;
fn main() {
    // let file = File::open("data.txt");

    match File::open("data.txt") {
        Ok(file) => println!("File opened"),
        Err(error) => println!("Failed: {}", error),
    }
}*/

/*fn main() {
    let a: f32 = 4.7;
    let b: f32 = 3.1;

    match safe_divide(4.4, 3.7) {
        Ok(n) => println!("Result of {} / {} is {:.2}", a, b, n),
        Err(e) => println!("Error:{}", e),
    }
}

fn safe_divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}*/

/*fn main() {
    let age: i32 = 20;
    match check_age(20) {
        Ok(n) => println!("Result of {:?}", n),
        Err(e) => println!("Error:{}", e),
    }
}

fn check_age(age: i32) -> Result<(i32), String> {
    if age < 0 {
        Err(String::from("Invalid age"))
    } else {
        Ok((age))
    }
}*/

/*fn main() {
    for i in 1..11 {
        println!("{}", i)
    }
}*/

/*fn main() {
    for i in 1..10 {
        println!("{}", i)
    }
}*/

/*fn main() {
    for i in (1..11).rev() {
        println!("{}", i);
    }
}*/

/*fn main() {
    for i in 1..21 {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}*/

/*fn main() {
    for i in 1..21 {
        if i % 2 != 0 {
            println!("{}", i);
        }
    }
}*/

/*fn main() {
    let mut sum = 0;
    for i in 1..101 {
        sum += i;
    }
    println!("Sum = {}", sum);
}*/

/*fn main() {
    let mut sum = 0;
    for i in 1..101 {
        sum += i;
        if i % 2 == 0 {
            println!("Sum = {}", sum);
        }
    }
}*/

/*fn main() {
    let mut product = 1;
    for i in 1..6 {
        product *= i;
        println!("Product = {}", product);
    }
}*/

/*fn main() {
    let mut n = 5;
    for i in 1..6 {
        while n > 0 {
            println!("{}", n);
            n -= 1;
        }
    }
}*/

/*fn main() {
    let mut x = 1;
    for i in 1..101 {
        while x < 100 {
            x *= 2;
            println!("X = {}", x);
        }
    }
}*/

/*fn main() {
    let mut count = 1;
    loop {
        println!("{}", count);
        if count == 5 {
            break;
        }
        count += 1;
    }
}*/

/*fn main() {
    let result = loop {
        break 42;
    };
    println!("{}", result);
}*/

/*fn main() {
    for i in 1..6 {
        for j in 1..6 {
            println!("{} * {} = {}", i, j, i * j);
        }
    }
}*/

/*fn main() {
    for i in 1..6 {
        for _ in 0..i {
            println!("*");
            println!("**");
            println!("*");
            println!("**");
            println!("***");
        }
        println!();
    }
}*/

/*fn main() {
    let mut largest_number = 0;
    for i in 1..101 {
        if i % 7 == 0 && i > largest_number {
            largest_number = i;
        }
    }
    println!("Largest_number: {}", largest_number);
}*/

/*fn main() {
    let mut num = 12345;
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    println!("Count: {}", count);
}*/

/*fn main() {
    let mut num = 12345;
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    println!("Sum: {}", sum);
}*/

/*fn main() {
let mut balance = 1000;
loop {
    println!("1. Deposit");
    println!("2. Withdraw");
    println!("3. Check balance");
    println!("4. Exit");
    let choice = 1; // get user input
    match choice {
        1 => {
            let balance = 500; // get user input
            balance += balance;

        2 => {
            let balance = 200; // get user input
            if balance <= balance {
                balance -= amount;
            } else {
                println!("Insufficient funds");

            }
            }
        }
        3 => {

        } println!("Balance:{}", balance);
        4 => break,
        _=> println!("Invalid choice"),
    }
}*/

use std::io::{self, Write};

// 4. Required Functions
fn calculate_total(total: i32, score: i32) -> i32 {
    total + score
}

fn calculate_average(total: i32, subjects: i32) -> f32 {
    (total as f32) / (subjects as f32)
}

fn determine_grade(avg: f32) -> String {
    // 5. Constraint: No match statements (use if/else only)
    if avg >= 80.0 {
        String::from("Excellent")
    } else if avg >= 70.0 {
        String::from("Very Good")
    } else if avg >= 60.0 {
        String::from("Good")
    } else if avg >= 50.0 {
        String::from("Pass")
    } else {
        String::from("Fail")
    }
}

fn main() {
    // A. Student Information
    let mut name = String::new();
    print!("Enter Student Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    print!("Enter Age: ");
    io::stdout().flush().unwrap();
    let mut age_str = String::new();
    io::stdin().read_line(&mut age_str).unwrap();
    let age: u32 = age_str.trim().parse().unwrap_or(0);

    if age == 0 {
        println!("Error: Age must be > 0. Exiting.");
        return;
    }

    print!("Enter Number of Subjects: ");
    io::stdout().flush().unwrap();
    let mut sub_str = String::new();
    io::stdin().read_line(&mut sub_str).unwrap();
    let num_subjects: i32 = sub_str.trim().parse().unwrap_or(0);

    if num_subjects < 1 {
        println!("Error: Number of subjects must be ≥ 1. Exiting.");
        return;
    }

    // B. Score Collection
    let mut total_score = 0;
    let mut passed_subjects: i32 = 0;
    let mut highest_score: i32 = 0;
    for i in 1..=num_subjects {
        loop {
            print!("Enter score for subject {}: ", i);
            io::stdout().flush().unwrap();
            let mut score_str = String::new();
            io::stdin().read_line(&mut score_str).unwrap();
            let score: i32 = score_str.trim().parse().unwrap_or(-1);

            if score >= 0 && score <= 100 {
                if score >= 50 {
                    passed_subjects += 1;
                }
                if score > highest_score {
                    highest_score = score;
                }

                total_score = calculate_total(total_score, score);
                break;
            } else {
                println!("Invalid score. Please enter a value between 0 and 100.");
            }
        }
    }

    // C. Processing
    let average = calculate_average(total_score, num_subjects);
    let grade = determine_grade(average);

    /* E. Output Format
        println!("\n--- STUDENT REPORT ---");
        println!("Name: {}", name);
        println!("Age: {}", age);
        println!("Subjects: {}", num_subjects);
        println!("Total Score: {}", total_score);
        println!("Average: {:.2}", average);
        println!("Grade: {}", grade);
        println!("----------------------");
    }*/
    student_report(
        name,
        age,
        num_subjects,
        total_score,
        average,
        &grade,
        passed_subjects,
        highest_score,
    )
}
// fn m{
//   let Name = "Tina";
//  let Age = 17;
//  let subjects = 5;
//  let total: i32 = 370;
//  let average = (total as f64) / (subjects as f64);
//  let Grade ="good";

// Calling the function

fn student_report(
    name: &str,
    age: u32,
    subject: i32,
    total: i32,
    average: f32,
    grade: &str,
    passed_subjects: i32,
    highest_score: i32,
) {
    println!("student_report");
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Subjects: {}", subject);
    println!("Total: {}", total);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);
    println!("Passed subject: {}", passed_subjects);
    println!("Highest_score: {}", highest_score);
}
