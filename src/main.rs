use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let number = rand::thread_rng().gen_range(1..=100);
    loop {
    println!("Please input your guess");
    let str = String::from("Hello");
    change(str);
    //println!("{}",str);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed: {guess}");
    match guess.cmp(&number) {
        Ordering::Less => println!("Too low"),
        Ordering::Greater => println!("Too High"),
        Ordering::Equal => {
            println!("You Win!");
            break;
        }
    }
    }

}

fn change(mut str : String){
    str.push_str("world");
    println!("{}",str);
}
