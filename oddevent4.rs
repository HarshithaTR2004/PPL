fn main() {
    let mut input = String::new();
    println!("Enter a number:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    if number%2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd.");
    }
}

