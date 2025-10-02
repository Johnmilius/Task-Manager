use crate::task::Task;

pub fn complete_task(tasks: &mut Vec<Task>, id: u32) -> bool {
    for task in tasks.iter_mut() {
        if task.id == id {
            task.mark_complete();
            return true;
        }
    }
    return false;
}

pub fn add_task(tasks: &mut Vec<Task>, new_task: Task) {
    tasks.push(new_task);
}

pub fn display_all_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No Tasks found.");
    } else {
        println!("All Tasks:");

        for task in tasks {
            println!("{}", task.display());
        }
    }
}


