#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;



fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("We need an integer");
    println!("Generating genesis Block!... ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Rewared");
        println!("0) Exit");
        print!("Enter your Choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
          0 => {
            println!("Exiting!");
            process::exit(0);
          },
          1 => {
            let mut sender = String::new();
            let mut receiver = String::new();
            let mut amount = String::new();
            
            print!("Enter sender address");
            io::stdout().flush();
            io::stdin().read_line(&mut sender);
            print!("Enter receiver address: ");
            io::stdout().flush();
            io::stdin().read_line(&mut receiver);
            print!("Enter amount: ");
            io::stdout().flush();
            io::stdin().read_line(&mut amount);

            let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());
            match res{
                true => println!("Transaction added"),
                false => println!("Transaction failed"),
            }
          },
          2 => {
              println!("Generating Block");
              let res = chain.generate_new_block();
              match res{
                true => println!("Block generated successfully"),
                false => println!("Block generation failed"),
              }
          }
          3 => {
            let mut new_diff = String::new();
            print!("Enter new difficulty: ");
            io::stdout().flush();
            io::stdin().read_line(&mut new_diff);
            chain.update_difficulty(new_diff.trim().parse().unwrap());
            let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
            match res{
                true => println!("Updated Difficulty"),
                false => println!("Failed Updated Difficulty"),
            }
          }
          4 => {
            let mut new_reward = String::new();
            print!("Enter new reward: ");
            io::stdout().flush();
            io::stdin().read_line(&mut new_reward);
            chain.update_reward(new_reward.trim().parse().unwrap());
            let res = chain.update_reward(new_reward.trim().parse().unwrap());
            match res {
                true => println!("Updated Reward"),
                false => println!("Failed Updated reward"),
            }
          }
          _ => println!("Invalid option please retry"),
        }
    }
}


