use card_deck::*;

fn main() {
    let your_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };

    println!("Your card is {:?}", &your_card);

    if card_deck::winner_card(&your_card) {
        println!("You are the winner!");
    }
}