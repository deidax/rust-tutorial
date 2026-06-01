use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
    let greeting: &str = "Guess The Number!";
    let winner_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("{}", greeting);
    println!("{}", "*".repeat(greeting.len()));
    println!("Winner number {winner_number}");
   

    loop {
        
        println!("[*] Enter a number: ");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read number");

        let guess: u32 = guess.trim().parse().expect("Please type a number");


        println!("==> {}", guess);
        match guess.cmp(&winner_number) {
            Ordering::Greater => println!("Nope..Too big"),
            Ordering::Less => println!("Nope..Too small"),
            Ordering::Equal => { 
                println!("Wiiiiineeeer!!!!!");
                break;
            },
        }
    }
    

}
