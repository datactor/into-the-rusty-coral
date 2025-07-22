//use clap::builder::Str;
use clap::{Arg, Command};
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;
use chrono::*;


#[derive(Debug)]
struct Todo {
    id: usize,
    title: String,
    creat_time: String,
    start_time: String,
    finish_time: String,
    scheduled_time: String,
    deadline: String,
    state: State,
}
#[derive(Debug)]
enum State {
    Done,
    Pending,
    Ongoing,
    Notstarted,
    Overdue,
    Shouldstart,
}

impl Todo {
    fn new(id: usize, title: String, creat_time: String, scheduled_time: String, deadline: String) -> Self {
        Self {
            id,
            title,
            creat_time, 
            start_time: String::from("Not start"),
            finish_time: String::from("Not finish"),
            scheduled_time,
            deadline,
            state: State::Notstarted,
        }
    }
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
                .arg(Arg::new("title").required(true).help("Task name"))
                .arg(Arg::new("scheduled").short('c').long("scheduled"))
                .arg(Arg::new("deadline").short('d').long("deadline"))
        )
        .subcommand(
            Command::new("list").about("Show all tasks"))
        .get_matches();

    let mut tasks = load_tasks();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let none = String::from("None");
            let id = tasks.len();
            let title = sub_m.get_one::<String>("title").unwrap();
            let scheduled_time = sub_m.get_one::<String>("scheduled").unwrap_or(&none);
            let deadline = sub_m.get_one::<String>("deadline").unwrap_or(&none);
            let creat_time = Local::now().format("%Y-%m-%d %H:%M:%S"); //"%Y-%m-%d %H:%M:%S"
            
            tasks.push(Todo::new(id, title.to_string(), creat_time.to_string(), scheduled_time.to_string(), deadline.to_string()));
            save_tasks(tasks);
        }
        Some(("list", _)) => {
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                println!("ID.   title.         creat_time.            start_time.            finish_time.            scheduled_time.            deadline.            done.");
                for task in &tasks {
                    println!("{}.   {}.   {}.   {}.   {}.   {}.   {}.   {:?}.", task.id, task.title, task.creat_time, task.start_time, task.finish_time, task.scheduled_time, task.deadline, task.state);
                }
            }
        }
        _ => unreachable!(),
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
        "{{\"id\": {}, \"title\": {}, \"creat_time\": {}, \"start_time\": {}, \"finish_time\": {}, \"scheduled_time\": {}, \"deadline\": {}, \"state\": {:?}}}",
        task.id, task.title, task.creat_time, task.start_time, task.finish_time, task.scheduled_time, task.deadline, task.state
    )
}

fn from_json(s: &str) -> Option<Todo> {
    let s = s.trim();
    if !s.starts_with('{') || !s.ends_with('}') {
        return None;
    }

    let mut id = 0;
    let mut title = String::new();
    let mut creat_time = String::new();
    let mut start_time = String::new();
    let mut finish_time = String::new();
    let mut scheduled_time = String::new();
    let mut deadline = String::new();
    let mut state = "Done";
    let state_e: State;

    for todo in s[1..s.len() - 1].split(',') {
        let parts: Vec<_> = todo
            .split("\":")
            .map(|x| x.trim().trim_matches('"'))
            .collect();

        if parts.len() != 2 {
            continue;
        }

        match parts[0] {
            "id" => id = parts[1].parse().unwrap_or(0),
            "title" => title = parts[1].to_string(),
            "creat_time" => creat_time = parts[1].to_string(),
            "start_time" => start_time = parts[1].to_string(),
            "finish_time" => finish_time = parts[1].to_string(),
            "scheduled_time" => scheduled_time = parts[1].to_string(),
            "deadline" => deadline = parts[1].to_string(),
            "state" => state = parts[1],
            _ => {}
        }
    }
    match state {
            "Done" => state_e = State::Done,
            "Pending" => state_e = State::Pending,
            "Ongoing" => state_e = State::Ongoing,
            "Notstarted" => state_e = State::Notstarted,
            "Overdue" => state_e = State::Overdue,
            "Shouldstart" => state_e =State::Shouldstart,
            _ => state_e = State::Done,
    };

    if title.is_empty() {
        None
    } else {
        Some(Todo { id, title, creat_time, start_time, finish_time, scheduled_time, deadline, state:state_e })
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


