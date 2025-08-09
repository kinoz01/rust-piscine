use std::rc::Rc;
use drop_the_thread::*;

fn main() {
    let pool = ThreadPool::new();
    let (id, thread) = pool.new_thread(String::from("command"));
    let (id1, thread1) = pool.new_thread(String::from("command1"));

    thread.skill();

    println!("{:?}", (pool.is_dropped(id), id, &pool.drops));

    thread1.skill();
    println!("{:?}", (pool.is_dropped(id1), id1, &pool.drops));

    let (id2, thread2) = pool.new_thread(String::from("command2"));
    let thread2 = Rc::new(thread2);
    let thread2_clone = thread2.clone();

    drop(thread2_clone);

    println!("{:?}", (pool.is_dropped(id2), id2, &pool.drops, Rc::strong_count(&thread2)));
}