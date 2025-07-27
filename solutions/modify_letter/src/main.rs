use modify_letter::*;

fn main() {
    println!("{}", remove_letter_sensitive("hEey hEey", 'e'));
    println!("{}", remove_letter_insensitive("hEye", 'e'));
    println!("{}", swap_letter_case("BbBb", 'b'));
}

