//user input 
use std::io;

use rand::Rng;
use std::cmp::Ordering;
fn main() {
    
    let secret_n = rand::thread_rng().gen_range(1..=100);
  //  println!("skriveni je broj: {secret_n}");

loop{
    println!("Unesi broj:");

    //mut for mutable 
    //varijable nisu mutable

    let mut guess = String::new();

    //handle user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Ne mogu da procitam lol");

    //println!("Uneli ste: {guess}");

    //konvertivanje stringa u unsigned32 varijablu
    //opet let jer shadowing
    //trim eliminise whitespace jer read_line() dodaje whitespace na kraju 
    //parse konvertuje
    //expect ukoliko unesemo nesto sto nije broj ret err msg
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
     };

    match guess.cmp(&secret_n) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal =>{
            println!("You win!");
            break;
        }
    }
}
}
