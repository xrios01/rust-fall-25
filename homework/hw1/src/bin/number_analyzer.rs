// Assignment 2: Number Analyzer

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Array of 10 integers
    let numbers: [i32; 10] = [3, 5, 6, 10, 15, 22, 30, 31, 42, 50];

    // print Fizz/Buzz/FizzBuzz, else even/odd
    for n in numbers {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n}: Fizz");
        } else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else {
            if is_even(n) {
                println!("{n}: even");
            } else {
                println!("{n}: odd");
            }
        }
    }


    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers = {sum}");

    // loop to find largest number
    let mut max_val = numbers[0];
    let mut idx: usize = 0;
    loop {
        if numbers[idx] > max_val {
            max_val = numbers[idx];
        }
        idx += 1;
        if idx >= numbers.len() {
            break;
        }
    }
    println!("Largest number is {max_val}");
}
