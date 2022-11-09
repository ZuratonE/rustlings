// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.


fn call_me(t: (i32, u64, f32)) {
    println!("{} {} {}", t.0, t.1, t.2);
}

fn main() {
    let tup: (i32, u64, f32) = (500, 5, 1.0);
    call_me(tup);

    let a: [u32; 3] = [0, 2, 3];
    let b = [0; 10];
    println!("a0 = {}, b[9] = {}", a[0], b[9]);

    //let mut idx = String::new();
    //std::io::stdin().read_line(&mut idx).expect("Failed to readline!");
    //let idx: usize = idx
    //    .trim()
    //    .parse()
    //    .expect("Failed to parse!");
    //
    //let el = b[idx];
    //println!("el = {el}");
}
