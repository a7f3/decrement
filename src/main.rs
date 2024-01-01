use rand::Rng;
use std::io;
use std::process;

fn main() {
    let mut hp: i32 = 100;
    let mut line = String::new();

    println!("pid: {}", process::id());

    while hp < 1000 {
        println!("hp is currently {}", hp);

        println!("Next? ");

        let _ = io::stdin().read_line(&mut line);

        hp -= rand::thread_rng().gen_range(2..5);

        if hp <= 0 {
            println!("Oh no you died! reviving you!");
            hp = 100;
        }
    }
    println!("You win!");
}
