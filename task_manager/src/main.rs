use crate::operations::*;
use crate::task::Task;
use std::io::{self, Write};

mod operations;
mod task;

fn main() {
    let mut running = true;
    // Create an empty vector to store tasks

    let mut tasks: Vec<Task> = vec![];

    while running {
        print!(
            "Menu:\n   a) Add Task\n   c) Complete Task\n   v) View Tasks\n   e) Exit Program\n   > "
        );
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line.");

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "a" => {
                println!("");
                print!("What is the task you wish to add?\n   > ");
                io::stdout().flush().expect("Failed to flush stdout");

                let mut new_description = String::new();
                io::stdin()
                    .read_line(&mut new_description)
                    .expect("Failed to read line.");

                let new_description = new_description.trim().to_string();
                let id = tasks.len() as u32 + 1;
                let new_task = Task::new(id, new_description);

                add_task(&mut tasks, new_task);
                println!("New Task Created Successfully");
            }
            "c" => {
                println!("Which task would you like to mark as completed?");

                display_all_tasks(&tasks);

                print!("  > ");
                io::stdout().flush().expect("Failed to flush stdout");

                // Reads the user input
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read the line.");

                let id_input = id_input
                    .trim()
                    .parse::<u32>()
                    .expect("Failed to parse Int. Try Again.");

                //Sends input into the function
                complete_task(&mut tasks, id_input);
                println!("Task, {},  has been set to completed.", id_input);
            }
            "v" => {
                display_all_tasks(&tasks);
            }
            "e" => {
                println!("GoodBye!");
                break;
            }
            _ => {
                println!("Invalid Input try again.")
            }
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
