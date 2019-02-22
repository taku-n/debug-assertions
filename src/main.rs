fn main() {
    if cfg!(debug_assertions) {
        dbg!("Debug Mode in main()");
    } else {
        println!("Not Debug Mode in main()");
    }

    f();
}

#[cfg(debug_assertions)]
fn f() {
    dbg!("Debug Mode in f()");
}

#[cfg(not(debug_assertions))]
fn f() {
    println!("Not Debug Mode in f()");
}
