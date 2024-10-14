use std::io::{stdin, BufRead};

struct Solders{
    solder_people: u32,
    money: u128,
}

fn main() {
    let std= stdin();
    let mut buff= std.lock();
    let string_input= buff.lines().next().unwrap().unwrap();

    let number= string_input.parse::<u32>().unwrap();
    if number >= 10 {
        println!("you win in a war");
    }else{
        println!("You lose in a war");
    }
}
