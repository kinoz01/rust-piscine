use std::io;

fn main() {
    let mut c = 0;
    loop {
        let mut input = String::new();
        c += 1;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin().read_line(&mut input).expect("Error");
        if input.trim() == "The letter e" {
            println!("Number of trials: {}", c);
            break;
        }
    }
}
