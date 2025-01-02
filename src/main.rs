use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("To do list using rust!");
}


// Command
// 1 - Add
// 2 - Remove
// 3 - Edit

// Argument pattern
// ---------------------- Add Item ----------------
// cargo run <Add> <Value>
