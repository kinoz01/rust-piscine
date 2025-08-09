use std::rc::Rc;

use ref_cell::*;

fn main() {
    let v = Rc::new(1); // we have one reference to this Rc

    // initialize the tracker, with the max number of
    // called references as 10
    let track = Tracker::new(10);

    let _v = Rc::clone(&v); // |\
    let _v = Rc::clone(&v); // | -> increase the Rc to 4 references
    let _v = Rc::clone(&v); // |/

    // take a peek of how much we already used from our quota
    track.peek(&v);

    let _v = Rc::clone(&v); // |\
    let _v = Rc::clone(&v); // |  -> increase the Rc to 8 references
    let _v = Rc::clone(&v); // | /
    let _v = Rc::clone(&v); // |/

    // this will change the tracker's inner value
    // and make a verification of how much we already used of our quota
    track.set_value(&v);

    let _v = Rc::clone(&v); // increase the Rc to 9 references
    let _v = Rc::clone(&v); // increase the Rc to 10 references, the maximum we allow

    track.set_value(&v);

    let _v = Rc::clone(&v); // surpass the maximum allowed references

    track.peek(&v);
    track.set_value(&v);

    track
        .messages
        .borrow()
        .iter()
        .for_each(|msg| println!("{}", msg));
}