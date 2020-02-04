use std::io; // input and output library
use rand::Rng; // for random math
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
     
        println!("please input your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    // assert!(guess == secret_number.to_string(), true);
}
