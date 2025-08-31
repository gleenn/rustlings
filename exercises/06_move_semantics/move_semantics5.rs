#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
// fn get_char(data: String) -> char {
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// fn string_uppercase(mut data: &String) {
fn string_uppercase(data: &String) -> &String {
    let upper_data = data.to_uppercase();

    println!("{upper_data}");
    data
}

fn main() {
    let data = "Rust is great!".to_string();

    // get_char(data);
    get_char(&data);

    // string_uppercase(&data);
    string_uppercase(&data);

    get_char(&data);
}

// In Glenn's words, ownership is deciding when memory should be cleaned up.
// So if you own a value, when the scope you're looking at ends, all the things you own die.
// So in effect, your program has a chain of functions that pass ownership of data between them.
// Each function takes ownership of the data, processes it, and then passes ownership to the next function.
// When the last function in the chain finishes, the data is cleaned up.
// This ensures that memory is managed efficiently and prevents memory leaks.
//
// Man AI stole the show after I wrote "chain of functions".
