// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 3);
    }
}

fn main() {
    call_me(3);
}
