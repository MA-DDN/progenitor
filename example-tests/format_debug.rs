use std::fs;

fn main() {
    let content = fs::read_to_string("debug_test_tokens.rs").expect("Failed to read debug tokens");
    
    // Replace some common patterns to make it more readable
    let formatted = content
        .replace(" ; ", ";\n")
        .replace(" { ", " {\n")
        .replace(" } ", "\n}\n")
        .replace(" :: ", "::")
        .replace(" , ", ", ");
    
    println!("{}", formatted);
}
