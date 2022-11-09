// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let s = String::from("hello");  // s comes into scope

    borrows(&s);
    println!("{}", s);
    takes_ownership(s);             // s's value moves into the function...
    //println!("{}", s); will fail
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn borrows(some_string: &String) { // some_string comes into scope
    println!("borrowed \"{}\"", some_string);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("taken \"{}\"", some_string);
}
