use clap::{Arg, Command};
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Todo {
    id: usize,
    title: String,
    done: bool,
}

impl Todo {
    fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            done: false,
        }
    }
}

fn get_data_file() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push("todo.json");
    path
}

fn load_tasks() -> Vec<Todo> {
    let path = get_data_file();
    if path.exists() {
        let data = fs::read_to_string(path).unwrap();
        let data = data
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'));

        if let Some(s) = data {
            s.split("},")
                .filter_map(|s| {
                    let x = if s.ends_with('}') {
                        s.to_string()
                    } else {
                        format!("{}}}", s)
                    };
                    from_json(&x)
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn to_json(task: &Todo) -> String {
    format!(
        "{{\"id\": {}, \"title\": {}, \"done\", {}}}",
        task.id, task.title, task.done
    )
}

fn from_json(s: &str) -> Option<Todo> {
    let s = s.trim();
    if !s.starts_with('{') || !s.ends_with('}') {
        return None;
    }

    let mut id = 0;
    let mut title = String::new();
    let mut done = false;

    for todo in s[1..s.len() - 1].split(',') {
        let parts: Vec<_> = todo
            .split(':')
            .map(|x| x.trim().trim_matches('"'))
            .collect();

        if parts.len() != 2 {
            continue;
        }

        match parts[0] {
            "id" => id = parts[1].parse().unwrap_or(0),
            "title" => title = parts[1].to_string(),
            "done" => done = parts[1] == "true",
            _ => {}
        }
    }

    if title.is_empty() {
        None
    } else {
        Some(Todo { id, title, done })
    }
}

fn save_tasks(tasks: Vec<Todo>) {
    let path = get_data_file();
    let contents = format!(
        "[{}]",
        tasks
            .iter()
            .map(|task| to_json(task))
            .collect::<Vec<String>>()
            .join(", ")
    );

    fs::write(path, contents).unwrap()
}

fn main() {
    let matches = Command::new("todo")
        .version("0.1.0")
        .about("A simple todo CLI app")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(Arg::new("title").required(true).help("Task name")),
        )
        .subcommand(Command::new("list").about("Show all tasks"))
        .get_matches();

    let mut tasks = load_tasks();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let title = sub_m.get_one::<String>("title").unwrap();
            let id = tasks.len();

            tasks.push(Todo::new(id, title.to_string()));
            save_tasks(tasks);
            println!("Task added: (id: {}): \"{}\"", id, title);
        }
        Some(("list", _)) => {
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                for task in &tasks {
                    println!("{}. {} [{}]", task.id, task.title, task.done);
                }
            }
        }
        _ => unreachable!(),
    }
}
