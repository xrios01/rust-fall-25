// Assignment 3: Guessing Game

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Secret number 
    let mut secret: i32 = 13;

    let mut guess: i32 = 1;   // guess
    let mut attempts: i32 = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess #{attempts}: {guess} — Correct!");
            break;
        } else if result == 1 {
            println!("Guess #{attempts}: {guess} — Too high!");
        } else {
            println!("Guess #{attempts}: {guess} — Too low!");
        }

        // next simulated guess
        guess += 1;
    }

    println!("It took {attempts} guesses.");

    
}
