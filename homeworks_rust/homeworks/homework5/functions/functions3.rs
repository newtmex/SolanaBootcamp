// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    call_this(1);
}

fn call_this(num: u32) {
    for i in 0..num {
        println!("Loop now {}", i + 1);
    }
}
