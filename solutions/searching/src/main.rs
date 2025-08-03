use searching::*;

fn main() {
    let ar = [1, 3, 4, 6, 8, 9, 11, 8];
    let f = search(&ar, 8);
    println!(
        "the element 8 is last in the position {:?} in the array {:?}",
        f, ar
    );
}