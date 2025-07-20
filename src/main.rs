use std::io::{self, Write};

#[derive(Debug)]
struct Todo {
    name: String,
    done: bool,
}

impl Todo {
    fn new(name: String) -> Self {
        Self { name, done: false }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    loop {
        println!("\n=== Menu ===");
        println!("1: Add task");
        println!("2: List tasks");
        println!("0: Exit");

        let input = read_input("Choose an option: ");

        match input.parse::<u32>() {
            Ok(1) => {
                let task_name = read_input("Enter task name: ");
                todo_list.push(Todo::new(task_name));
                println!("Task added.");
            }
            Ok(2) => {
                if todo_list.is_empty() {
                    println!("No tasks yet.");
                } else {
                    println!("\n--- Tasks ---");
                    for (i, task) in todo_list.iter().enumerate() {
                        println!(
                            "{}: {} [{}]",
                            i + 1,
                            task.name,
                            if task.done { "✓" } else { " " }
                        );
                    }
                }
            }
            Ok(0) => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}
