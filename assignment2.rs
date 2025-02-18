fn main() {
    let numbers = [4, 3, 4, 5, 7, 60, 45, 12, 4, 14];

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

  
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Find and print the largest number
    let mut max_num = numbers[0];
    for &num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }
    println!("Largest number: {}", max_num);
}
