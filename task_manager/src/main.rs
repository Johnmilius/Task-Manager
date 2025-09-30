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

fn add_task(newtask: task){

    

}

fn main() {
    // OLD WAY (what you have now):
    // let task1 = Task {
    //     id: 1,
    //     description: "Clean the Dishes".to_string(),
    //     completed: false,
    // };
    let tasks = vec![];

    // NEW WAY using the constructor:
    let task1 = Task::new(1, "Clean the Dishes".to_string());
    let task2 = Task::new(2, "Buy groceries".to_string());
    let task3 = Task::new(3, "Finish Rust homework".to_string());


    for task in &tasks {
        if task.completed {
            println!("Task {}: {} [DONE]", task.id, task.description);
        } else {
            println!("Task {}: {} [TODO]", task.id, task.description);
        }
    }
}
