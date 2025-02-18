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
    let secret = 2; 
    let mut guess;
    let mut attempts = 0;

    loop {
      
        guess = [0, 4, 1, 2][attempts]; 
        attempts += 1;

        let result = check_guess(guess, secret);
        if result == 0 {
            println!("Correct! The secret number is {}", secret);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }
    }

    println!("You guessed the correct number in {} attempts!", attempts);
}




