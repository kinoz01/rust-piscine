use tic_tac_toe::*;

fn main() {
    println!(
        "{}",
        tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
    );
    // tie
    println!(
        "{}",
        tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
    );
    // player O won

    let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

    println!("{}", tic_tac_toe(diag));
    // player X won

    let mut s1 = String::from("HELLO");
    let s2 = String::new();

    s1.push_str(s2.as_str());
}

