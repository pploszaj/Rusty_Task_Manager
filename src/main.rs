use std::io;

struct Task {
    id: usize,
    title: String,
    description: Option<String>,
}

fn add_task(tasks: &mut Vec<Task>, task_input: String, description_input: String) {
    let description = if description_input == "N" { None } else { Some(description_input) };

    tasks.push(Task {
        id: tasks.len(),
        title: task_input,
        description: description
    });
}

fn view_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!("Task ID: {} \nTask name: {}", task.id, task.title);
        match &task.description {
            Some(desc) => println!("Description: {}", desc),
            None => println!("Description: None")
        }
    }
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

    let task_input = read_input("\nWelcome to Rusty Task Manager. Please start by adding a task.\n");
    let description_input = read_input("\nWould you like to provide a short description? If not, type N and press Enter. If yes, just type the description.\n");
    add_task(&mut tasks, task_input, description_input);
    

    println!("\nGreat! Looks like you've successfully added your first task. Let's get you familiarized with the menu. Type 'View' if you'd like to see all your tasks. Type 'Add' if you'd like to add a new task. Type 'Update' to update or delete an exisiting task. Type 'Exit' to exit the program.\n");

    loop {
        let user_input = read_input("\nWhat would you like to do next?\n");
        if user_input == "View" {
            view_tasks(&tasks);
        } else if user_input == "Add" {
            let task_name = read_input("\nEnter name of task:\n");
            let desc_input = read_input("\nEnter description or N\n");
            add_task(&mut tasks, task_name, desc_input);

        } else if user_input == "Update" {
            loop {
                view_tasks(&tasks);
                let task_id = read_input("\nType in the ID of the task you'd like to update\n");
                match task_id.parse::<usize>() {
                    Ok(id) => {
                        if id < tasks.len() {
                            loop {
                                let updated_field = read_input("What would you like to update? Type T for title or D for description. B for both.");
                                if updated_field == "T" {
                                    let updated_task_name = read_input("\nEnter new name of task:\n");
                                    tasks[id].title = updated_task_name;
                                    break;
                                } else if updated_field == "D" {
                                    let updated_desc_input = read_input("\nEnter new description.\n");
                                    tasks[id].description = Some(updated_desc_input);
                                    break;
                                } else if updated_field == "B" {
                                    let updated_task_name = read_input("\nEnter new name of task:\n");
                                    let updated_desc_input = read_input("\nEnter new description.\n");
                                    tasks[id].title = updated_task_name;
                                    tasks[id].description = Some(updated_desc_input);
                                    break;
                                } else {
                                    println!("Incorrect input! Type T for title or D for description. B for both.");
                                    continue;
                                    }
                                }
                            println!("Update successful!");
                        } else {
                            println!("Task ID is out of range. Please enter a valid ID");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                    }
                }
                
            }
        } else if user_input == "Delete" {
            loop {
                let task_id = read_input("\nType in the ID of the task you'd like to delete\n");
                match task_id.parse::<usize>() {
                    Ok(id) => {
                        if id < tasks.len() {
                            tasks.remove(id);
                            break;
                        } else {
                            println!("The ID you entered doesn't exist. Please enter a valid task ID.");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                    }
                }
            }
            view_tasks(&tasks);
            println!("Successfully deleted task");
        } else if user_input == "Exit" {
            return;
        }
    }

}
