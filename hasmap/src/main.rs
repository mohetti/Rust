use std::collections::HashMap;
use std::io;

fn main() {
    // text interface to add employee names to a department
    // eg "Add Sally to engineering"
    let mut map = HashMap::new();

    println!("Welcome! Please state the employee and his or her department.");
    println!("Please use the format 'Add EMPLOYEE to DEPARTMENT'");
    let input = user_input();
    println!("input is: {:?}", input);
    map.entry(&input[3]).or_insert(&input[1]);
    let input1 = user_input();
    map.entry(&input1[3]).or_insert(&input1[1]);

    let input2 = user_input();
    map.entry(&input2[3]).or_insert(&input2[1]);
    println!("map is: {:?}", map);
}

fn user_input() -> Vec<String> {
    let mut input = String::new();
    let mut chunks: Vec<String>;
    loop {
        io::stdin().read_line(&mut input).unwrap();
        chunks = input.split_whitespace().map(str::to_string).collect();
        if chunks.len() == 4 && chunks[0] == "Add" && chunks[2] == "to" {
            return chunks;
        } else {
            input = String::new();
            continue;
        }
    }
}
