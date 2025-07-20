use std::process;

fn main() {
    let mut todo: Vec<Todo> = Vec::new();
    loop {
        let menu = menu_select();
        println!("You select number : {menu}");
        match menu {
            0 => process::exit(0),
            1 => todo.push(add_task()),
            2 => print_task(&todo),
            _ => println!("No menu"),
        }
    }
}

struct Todo {
    //name: String,
    contents: String,
    todo: bool,
}
impl Todo {
    fn new(content: String) -> Self {
        Self {
            contents: (content),
            todo: (false),
        }
    }
}

fn menu_select() -> u8 {
    loop {
        println!("1. Add task");
        println!("2. Show task");
        println!("0. Exit");
        println!("Please select a menu");
        let mut menu = String::new();
        std::io::stdin().read_line(&mut menu).expect("select menu");
        match menu.trim().parse::<u8>() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}

fn add_task() -> Todo {
    println!("Input your task!");

    let mut task = String::new();
    std::io::stdin()
        .read_line(&mut task)
        .expect("No input task");
    Todo::new(task.trim().to_string())
}

fn print_task(todo: &Vec<Todo>) {
    for list in todo {
        println!("{} : {}\n", list.contents, list.todo);
    }
}
