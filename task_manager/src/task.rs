use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)] // Add Serialize and Deserialize
pub struct Task {
    pub id: u32,
    pub description: String,
    completed: bool,
}

impl Task {
    // This is like a constructor
    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false, // Default to false
        }
    }

    pub fn is_completed(&self) -> bool {
        return self.completed;
    }

    pub fn status_str(&self) -> &str {
        if self.completed {
            return "[DONE]";
        } else {
            return "[TODO]";
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }

    pub fn display(&self) -> String {
        return format!(
            "   Task {}: {} {}",
            self.id,
            self.description,
            self.status_str(),
        );
    }
}
