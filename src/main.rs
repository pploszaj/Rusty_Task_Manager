use std::io;

struct Task {
    id: usize,
    title: String,
    description: Option<String>,
}


fn main() {
    //a vector that holds all the tasks
    let mut tasks: Vec<Task> = Vec::new();
    
    //take user input
    fn read_input(prompt: &str) -> String {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Unable to read input.");
        input.trim().to_string()
    }

    let new_task = read_input("Welcome to Rusty Task Manager. Please start by adding a task.");
    let description_input = read_input("Would you like to provide a short description? If not, type N and press Enter. If yes, just type the description.");

    let description = if description_input == "N" { None } else { Some(description_input) };

    tasks.push(Task {
        id: tasks.len(),
        title: new_task,
        description: description
    });

    println!("Great! Looks like you've successfully added your first task. Let's get you familiarized with the menu. Type 'View' if you'd like to see all your tasks. Type 'Add' if you'd like to add a new task. Type 'update' to update or delete an exisiting task. Type 'Exit' to exit the program.");

    loop {
        let user_input = read_input("What would you like to do next?");
        if user_input == "View" {
            for task in &tasks {
                println!("Task ID: {} \nTask name: {}", task.id, task.title);
                match &task.description {
                    Some(desc) => println!("Description: {}", desc),
                    None => println!("Description: None")
                }
            }
        }
        if user_input == "Exit" {
            return;
        }
    }


    
}
