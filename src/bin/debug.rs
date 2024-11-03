fn process_strings(strings: [&str; 3]) {
    for s in &strings {
        println!("{}", s);
    }
}

fn main() {
    let my_strings = ["Hello", "Hi"];
    process_strings(my_strings);
}