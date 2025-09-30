use std::{io::{self, Write}, path::PrefixComponent};

//working on complete task function

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    // This is like a constructor
    fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false, // Default to false
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, new_task: Task) {
    tasks.push(new_task);
}

fn complete_task(){}

fn main() {
    let mut running = true;
    // Create an empty vector to store tasks

    let mut tasks: Vec<Task> = vec![];

    while running {
        print!(
            "Menu:\n   a) Add Task\n   d) Delete Task\n   c) Complete Task\n   v) View Tasks\n   e) Exit Program\n   > "
        );
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line.");

        let input = input.trim();

        if input == "a" {
            println!("");
            print!("What is the task you wish to add?\n   > ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut new_Description = String::new();
            io::stdin()
                .read_line(&mut new_Description)
                .expect("Failed to read line.");

            let new_Decription = new_Description.trim().to_string();
            let id = tasks.len() as u32 + 1;
            let new_task = Task::new(id,new_Description);            

            add_task(&mut tasks, new_task);

            println!("New Task Created Successfully");

        } else if input == "d" {
            // May be complicated due to the incrementing id 
        } else if input == "c" {
            
        } else if input == "v" {
            println!("\nAll Tasks:");
            for task in &tasks{
                print!("Task: {}, {}", task.id, task.description);
            }
            println!("");
        } else if input == "e" {
            println!("GoodBye!");
            break;
        } else {
            println!("Invalid Input try again.")
        }
    }

    // Create tasks using the constructor:
    // let task1 = Task::new(1, "Clean the Dishes".to_string());
    // let task2 = Task::new(2, "Buy groceries".to_string());
    // let task3 = Task::new(3, "Finish Rust homework".to_string());

    // // Add tasks to the vector using our add_task function
    // add_task(&mut tasks, task1);
    // add_task(&mut tasks, task2);
    // add_task(&mut tasks, task3);
}
