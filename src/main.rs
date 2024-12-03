use std::io::{self, Write};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            description,
            completed: false,
        };
        self.tasks.push(task);
    }

    fn toggle_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = !task.completed;
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string(self)?;
        fs::write(filename, json)
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let todo_list: TodoList = serde_json::from_str(&contents)?;
        Ok(todo_list)
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let filename = "todo.json";

    // Try to load existing tasks
    if let Ok(loaded_list) = TodoList::load_from_file(filename) {
        todo_list = loaded_list;
    }

    loop {
        println!("\n=== Todo List Manager ===");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Toggle task status");
        println!("4. Save and quit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todo_list.add_task(description.trim().to_string());
            }
            "2" => {
                for (i, task) in todo_list.tasks.iter().enumerate() {
                    println!("{}. [{}] {}", 
                        i + 1,
                        if task.completed { "x" } else { " " },
                        task.description
                    );
                }
            }
            "3" => {
                print!("Enter task number to toggle: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    todo_list.toggle_task(index - 1);
                }
            }
            "4" => {
                todo_list.save_to_file(filename).unwrap();
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}
