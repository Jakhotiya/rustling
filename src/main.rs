use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game begins!");



    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
          Ok(num)=>num,
          Err(_)=>continue
        };

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too less"),
            Ordering::Greater=>println!("To high"),
            Ordering::Equal=>{
                println!("You win!!");
                break;
            }
        }
    }

}
