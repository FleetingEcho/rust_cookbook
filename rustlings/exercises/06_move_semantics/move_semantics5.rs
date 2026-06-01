#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// fn string_uppercase(mut data: String) {
//     data = data.to_uppercase();

//     println!("{data}");
// }
fn string_uppercase(data: &str) {
    let upper = data.to_uppercase();   // 存到新变量，不覆盖 data
    println!("{upper}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&data);
}
