use std::io::{stdin, BufRead};

#[derive(Debug)]
struct National{
    pub solder_people: u32,
    pub money: u128,
}

impl National {
    fn new() -> Self{
        National { solder_people: 100, money: 100000 }
    }
    fn win(self) -> Self{
        Self { solder_people: self.solder_people + 100, money: self.money + 1000 }
    }
    fn lose(self) -> Self{
        Self { solder_people: self.solder_people - 100, money: self.money - 1000 }
    }
    fn hikiwake(self) -> Self{
        Self { solder_people: self.solder_people - 50, money: self.money - 500 }
    }
}

fn input() -> String{
    let std= stdin();
    let mut buff= std.lock();
    let string_input= buff.lines().next().unwrap().unwrap();

    string_input
}

fn main() {
    let mut Kan_u= National::new();

    let enermy_number= input().parse::<u32>().unwrap();

    if enermy_number < Kan_u.solder_people {
        println!("you win in a war");
        Kan_u= Kan_u.win();
    }else if enermy_number == Kan_u.solder_people {
        println!("hikiwake");
        Kan_u= Kan_u.hikiwake();
    }else{
        println!("You lose in a war");
        Kan_u= Kan_u.lose();
    }

    println!("{:?}", Kan_u);
}
