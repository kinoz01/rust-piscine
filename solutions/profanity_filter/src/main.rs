use profanity_filter::*;

fn main() {
    ["hello there", "", "you are stupid", "stupid"]
        .into_iter()
        .for_each(|m| println!("{:?}", profanity_filter::check_ms(m)));
}