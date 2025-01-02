use std::env;
use std::io;
use std::io::stdin;


enum Operations {
    List,
    Add,
    Edit,
    Completed
}

fn perform_operation(operation: Operations){
    match operation {
        Operations::List => println!("You have selected List Operation!!!"),
        Operations::Add => println!("You have selected Add Operation!!!"),
        Operations::Edit => println!("You have selected Edit Operation!!!"),
        Operations::Completed => println!("You have selected the Completed Operations!!!")  
    }
}

fn main() {
    println!("Welcome to to Rust Todo List!");
    println!("\n------------ Select below Operations ------------------");
    println!("1 - Show todo list");
    println!("2 - Add items to todo list");
    println!("3 - Edit any item");
    println!("4 - Mark a item completed");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read value");
    let mut user:Operations;
    if input == "1" {
        user = Operations::List;
        perform_operation(user);
    }

}

// Command
// 1 - List  -- List all the item 
// 2 - Add
// 3 - Completed
// 4 - Edit

// Argument pattern
// cargo run show
// cargo run <Add> <Value>
// cargo run <Completed> <Index_Number>
// cargo run <Edit> <Index_Number> <New value>