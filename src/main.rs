use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use std::cmp::Reverse;

use chrono::{DateTime, Utc};
use std::time::Duration;
use std::time::UNIX_EPOCH;

/// Represents the available commands for the todo application.
#[derive(Debug, StructOpt)]
pub enum Command {
    /// Add a new todo item with a task and priority.
    #[structopt(name = "add")]
    Add {
        /// The task description for the new todo item.
        task: String,
        /// The priority level for the new todo item.
        priority: u64,
    },

    /// Remove a todo item by its ID
    #[structopt(name = "remove")]
    Remove {
        /// The unique identifier of the todo item to be removed.
        id: u64,
    },

    /// List all todo items.  
    #[structopt(name = "list")]
    List,

    /// Display help information about the todo application.
    #[structopt(name = "help")]
    Help,

    /// Clear all todo items.
    #[structopt(name = "clear")]
    Clear,

    /// Print out todo list based on priority level from highest to lowest.
    #[structopt(name = "prioritize")]
    Prioritize,

    /// Print out todo list based on when they where created from earliest to latest.
    #[structopt(name = "schedule")]
    Schedule,
    /// Edit the task of a todo item by providing a new task name and the ID of the todo item.
    #[structopt(name = "edit")]
    Edit {
        /// The new task description for the todo item.
        task: String,
        /// The unique identifier of the todo item to be edited.
        id: u64,
    },
}
/// Represents a todo item with associated details.
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    /// The unique identifier of the todo item.
    id: u64,
    /// The task description of the todo item.
    task: String,
    /// The priority level of the todo item (1-5 inclusive).
    priority: u64,
    /// The timestamp when the todo item was created.
    created: i64,
}
/// Represents a collection of todo items.
#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    /// The list of todo items stored in the todo list with a Vec.
    todos: Vec<Todo>,
}

impl TodoList {
    /// Creates a new `TodoList` instance with an empty list of todos.
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }
    /// Adds a new todo item to the todo list with the specified task and priority.
    ///
    /// # Arguments
    ///
    /// * `task` - The task description for the new todo item.
    /// * `priority` - The priority level for the new todo item.
    ///
    /// # Example
    ///
    /// ```
    /// let mut todo_list = TodoList::new();
    /// todo_list.add_todo("Complete the assignment", 3);
    /// ```
    fn add_todo(&mut self, task: &str, priority: u64) {
        let id = self.todos.len() as u64 + 1;
        let todo = Todo {
            id,
            task: task.to_string(),
            priority,
            created: Utc::now().timestamp(),
        };
        if priority <= 5 && priority > 0 {
            self.todos.push(todo);
        } else {
            println!(
                "Invalid priority: {} for task: {}. Not Added",
                priority, task
            );
        }
    }
    /// Removes a todo item from the todo list based on its ID and resets the IDs of the rest.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the todo item to be removed.
    ///
    /// # Example
    ///
    /// ```
    /// let mut todo_list = TodoList::new();
    /// todo_list.add_todo("Complete the assignment", 3);
    /// todo_list.remove_todo(1);
    /// ```
    fn remove_todo(&mut self, id: u64) {
        let size = self.todos.len();
        self.todos.retain(|todo| todo.id != id);
        if size == self.todos.len() {
            println!("Invalid ID. Nothing deleted.");
        } else {
            let mut new_ids: u64 = 1;
            for todo in &mut self.todos {
                todo.id = new_ids;
                new_ids += 1;
            }
        }
    }
    /// Clears all todo items from the todo list.
    ///
    /// # Example
    ///
    /// ```
    /// let mut todo_list = TodoList::new();
    /// todo_list.add_todo("Complete the assignment", 3);
    /// todo_list.clear_todo();
    /// ```
    fn clear_todo(&mut self) {
        self.todos.clear();
    }
    /// Displays the details of all todo items in the todo list.
    ///
    /// # Example
    ///
    /// ```
    /// let mut todo_list = TodoList::new();
    /// todo_list.add_todo("Complete the assignment", 3);
    /// todo_list.display_todos();
    /// ```
    fn display_todos(&self) {
        if self.todos.is_empty() {
            println!("No tasks left!");
        } else {
            for todo in &self.todos {
                let d = UNIX_EPOCH + Duration::from_secs(todo.created as u64);
                let datetime = DateTime::<Utc>::from(d);
                let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
                println!("{}: {}, created: {}", todo.id, todo.task, timestamp_str);
            }
        }
    }
    /// Edits the task of a todo item in the todo list.
    ///
    /// # Arguments
    ///
    /// * `new_task` - The new task description for the todo item.
    /// * `id` - The unique identifier of the todo item to be edited.
    ///
    /// # Example
    ///
    /// ```
    /// let mut todo_list = TodoList::new();
    /// todo_list.add_todo("Complete the assignment", 3);
    /// todo_list.edit_todo("Updated task", 0);
    /// ```
    fn edit_todo(&mut self, new_task: &str, id: u64) {
        if (id - 1) < self.todos.len() as u64 {
            self.todos[(id - 1) as usize].task = new_task.to_string();
        } else {
            println!("Invalid ID");
        }
    }
}
/// The main function of the command-line todo list application.
///
/// # Examples
///
/// ```
/// // Run the command-line todo list application
/// cargo run -- <command>
/// ```
fn main() {
    // Load the todo list from a file or create a new one if the file doesn't exist
    let todo_list: TodoList = load_todo_list().unwrap_or_else(TodoList::new);
    // Parse the command-line arguments into a Command enum
    let command = Command::from_args();
    // Execute the appropriate action based on the parsed command
    match command {
        // Add a new todo item to the list
        Command::Add { task, priority } => {
            let mut updated_todo_list = todo_list;
            updated_todo_list.add_todo(&task, priority);
            save_todo_list(&updated_todo_list);
        }
        // Remove a todo item from the list
        Command::Remove { id } => {
            let mut updated_todo_list = todo_list;
            updated_todo_list.remove_todo(id);
            save_todo_list(&updated_todo_list);
        }
        // Display the list of todos
        Command::List => {
            todo_list.display_todos();
        }
        // Display help information
        Command::Help => {
            display_help();
        }
        // Clear all todos from the list
        Command::Clear => {
            let mut updated_todo_list = todo_list;
            updated_todo_list.clear_todo();
            save_todo_list(&updated_todo_list);
        }
        // Prioritize and display todos
        Command::Prioritize => {
            let mut updated_todo_list = todo_list;
            updated_todo_list
                .todos
                .sort_by_key(|todo| Reverse(todo.priority));
            updated_todo_list.display_todos();
        }
        // Display todos by creation date
        Command::Schedule => {
            let mut updated_todo_list = todo_list;
            updated_todo_list.todos.sort_by_key(|todo| todo.created);
            updated_todo_list.display_todos();
        }
        // Edit the task of a todo item
        Command::Edit { task, id } => {
            let mut updated_todo_list = todo_list;
            updated_todo_list.edit_todo(&task, id);
            save_todo_list(&updated_todo_list);
        }
    }
}
/// Loads a todo list from a JSON file.
///
/// # Returns
///
/// Returns an `Option<TodoList>` containing the loaded todo list if the file exists;
/// returns `None` otherwise.
///
/// # Example
///
/// ```
/// let loaded_todo_list = load_todo_list();
/// if let Some(todo_list) = loaded_todo_list {
///     // Process the loaded todo list...
/// } else {
///     // No todo list file found.
/// }
/// ```
fn load_todo_list() -> Option<TodoList> {
    let path = Path::new("./todos.json");
    if path.exists() {
        let mut file = File::open(path).expect("Unable to open todo list file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read todo list file");
        let todo_list: TodoList = serde_json::from_str(&contents).expect("Unable to parse JSON");
        Some(todo_list)
    } else {
        None
    }
}
/// Saves a todo list to a JSON file.
///
/// # Arguments
///
/// * `todo_list` - The todo list to be saved.
///
/// # Example
///
/// ```
/// let todo_list = TodoList::new();
/// save_todo_list(&todo_list);
/// ```
fn save_todo_list(todo_list: &TodoList) {
    let path = Path::new("./todos.json");
    let serialized =
        serde_json::to_string_pretty(&todo_list).expect("Unable to serialize todo list");
    let mut file = File::create(path).expect("Unable to create todo list file");
    file.write_all(serialized.as_bytes())
        .expect("Unable to write todo list to file");
}
/// Displays help information about the command-line todo list application.
fn display_help() {
    println!(
        "simple command-line todo list

            USAGE:
                cargo run -- <command>
            ARGS:
                add <task-name> <priority>      Add a task to the list, include priority of task (1-5) inclusive
                remove <task-id>                Remove a task at the given index
                list                            List the todos
                clear                           Clear all the todos   
                prioritize                      List the todos in order of priority (highest to lowest)
                help                            Print help information
                schedule                        List the todos by the date they were created (in Utc)
                edit <id>                       Change the name of a task given id
        "
    );
}
#[cfg(test)]
mod tests {
    use crate::TodoList;
    #[test]
    fn test_clear() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("task 1", 1);
        todo_list.add_todo("task 2", 2);
        todo_list.clear_todo();
        assert_eq!(todo_list.todos.len(), 0);

        todo_list.clear_todo();
        assert_eq!(todo_list.todos.len(), 0);
    }
    #[test]
    fn test_add() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("task 1", 1);
        assert_eq!(todo_list.todos.len(), 1);

        todo_list.add_todo("Invalid task", 0);
        assert_eq!(todo_list.todos.len(), 1);

        todo_list.add_todo("Invalid task", 6);
        assert_eq!(todo_list.todos.len(), 1);
    }
    #[test]
    fn test_delete() {
        let mut todo_list = TodoList::new();
        todo_list.remove_todo(1);
        assert_eq!(todo_list.todos.len(), 0);

        todo_list.add_todo("task 1", 1);
        todo_list.add_todo("task 2", 2);
        todo_list.add_todo("task 3", 3);

        todo_list.remove_todo(0);
        assert_eq!(todo_list.todos.len(), 3);

        todo_list.remove_todo(4);
        assert_eq!(todo_list.todos.len(), 3);

        todo_list.remove_todo(1);
        assert_eq!(todo_list.todos.len(), 2);

        assert_eq!(todo_list.todos[0].task, "task 2");
        assert_eq!(todo_list.todos[1].task, "task 3");
    }
    #[test]
    fn test_edit() {
        let mut todo_list = TodoList::new();

        todo_list.add_todo("task 1", 1);
        todo_list.add_todo("task 2", 2);
        todo_list.add_todo("task 3", 3);

        todo_list.edit_todo("edited task", 1);
        assert_eq!(todo_list.todos[0].task, "edited task");

        todo_list.edit_todo("bad edited task", 4);
        assert_eq!(todo_list.todos[0].task, "edited task");
        assert_eq!(todo_list.todos[1].task, "task 2");
        assert_eq!(todo_list.todos[2].task, "task 3");
    }
}
