use std::io;

struct Task {
    id: i32,
    title: String,
    description: Option<String>,
}


fn main() {
    //a vector that holds all the tasks
    let mut vector: Vec<Task> = Vec::new();
    
    //take user input
    fn read_input(prompt: &str) -> String {
        let mut input = String::new();
        println!({}, prompt);
        io::stdin().read_line(&mut input).expect("Unable to read input.");
        input.trim().to_string()
    }

    
    
}
