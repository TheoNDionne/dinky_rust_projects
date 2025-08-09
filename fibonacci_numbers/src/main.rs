use std::io;

fn calculate_nth_fibonacci_number(target_fibonacci_index: u32) -> u64 {
    let mut counter: u32 = 0;

    if target_fibonacci_index != 0 {
        counter = 1; // ensures 0 does not crash
    }

    let mut tailing_fibonacci_number: u64 = 1;
    let mut leading_fibonacci_number: u64 = 1;
    let mut temp_fibonacci_number: u64;

    while counter < target_fibonacci_index {
        temp_fibonacci_number = leading_fibonacci_number.clone();
        leading_fibonacci_number += tailing_fibonacci_number;
        tailing_fibonacci_number = temp_fibonacci_number;

        counter += 1;
    }
    leading_fibonacci_number
}

fn main() {
    // get user input for desired Fibonacci number
    let mut input = String::new();
    println!("Enter the index of the desired Fibonacci number: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim(); // trim the input

    let target_fibonacci_index: u32 = input.parse().expect("Please type a number");

    // get the Fibonacci number by calling the public function
    let leading_fibonacci_number = calculate_nth_fibonacci_number(target_fibonacci_index);

    println!("Fibonacci number {target_fibonacci_index} is: {leading_fibonacci_number}");
}
