use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there is exactly one argument (the program name is the first argument)
    if args.len() != 2 {
        eprintln!("Usage: 0. パーンツ!!, || 1. 逃げろ!! || 2. かまれる~ {}", args[0]);
        std::process::exit(1);
    }

    println!("");
    println!("");
    println!("");

    // Retrieve the parameter
    let parameter = &args[1];

    let number: usize= parameter.parse().unwrap();

    match number {
        0_usize => println!("パーンツ!!"),
        1_usize => println!("逃げろ!!"),
        2_usize => println!("かまれる~"),
        _ => println!("nothing"),
    };
    
}
