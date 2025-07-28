use strings::*;

fn main() {
    println!("length of {} = {}", "❤", "❤".len());
    println!("length of {} = {}", "❤", char_length("❤"));
    println!("length of {} = {}", "形声字", char_length("形聲字"));
    println!("length of {} = {}", "形声字", "形聲字".len());
    println!("length of {} = {}", "change", "change".len());
    println!("length of {} = {}", "change", char_length("change"));
    println!("char length of {} = {}", "😍", char_length("😍"));
}
