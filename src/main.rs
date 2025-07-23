//use chrono::format;
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
    creat_time: Option<NaiveDateTime>,
    start_time: Option<NaiveDateTime>,
    finish_time: Option<NaiveDateTime>,
    scheduled_time: Option<NaiveDateTime>,
    deadline_time: Option<NaiveDateTime>,
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
    fn new(id: usize, title: String, creat_time: Option<NaiveDateTime>, scheduled_time: Option<NaiveDateTime>, deadline: Option<NaiveDateTime>) -> Self {
        Self {
            id,
            title,
            creat_time,
            start_time: None,
            finish_time: None,
            scheduled_time,
            deadline_time: deadline,
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
                .arg(Arg::new("scheduled").help("Y-M-D_H:M:S").value_parser(parse_datetime).short('c').long("scheduled"))
                .arg(Arg::new("deadline").help("Y-M-D_H:M:S").value_parser(parse_datetime).short('d').long("deadline"))
        )
        .subcommand(Command::new("list").about("Show all tasks"))
        .get_matches();

    let mut tasks = load_tasks();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let none = String::from("None");
            let id = tasks.len();
            let title = sub_m.get_one::<String>("title").unwrap();
            let scheduled = sub_m.get_one::<String>("scheduled").unwrap_or(&none);
            let scheduled_time: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&scheduled, "%Y-%m-%d_%H:%M:%S").ok();
            let deadline = sub_m.get_one::<String>("deadline").unwrap_or(&none);
            let deadline_time: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&deadline, "%Y-%m-%d_%H:%M:%S").ok();
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let creat_time= NaiveDateTime::parse_from_str(&now, "%Y-%m-%d %H:%M:%S").ok(); //%Y-%m-%d %H:%M:%S

            tasks.push(Todo::new(id, title.to_string(), creat_time, scheduled_time, deadline_time));
            save_tasks(tasks);
        }
        Some(("list", _)) => {
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                println!("ID.   title.              creat_time.          start_time.          finish_time.         scheduled_time.      deadline.            done.");
                for task in &tasks {
                    println!("{:<4} {:<20} {:<20} {:<20} {:<20} {:<20} {:<20} {:?}",
                    task.id, task.title, task.creat_time.map_or("None".to_string(), |dt| dt.to_string()), task.start_time.map_or("None".to_string(), |dt| dt.to_string()), task.finish_time.map_or("None".to_string(), |dt| dt.to_string()), task.scheduled_time.map_or("None".to_string(), |dt| dt.to_string()), task.deadline_time.map_or("None".to_string(), |dt| dt.to_string()), task.state);
                }
            }
        }
        _ => unreachable!(),
    }
}

fn parse_datetime(s: &str) -> Result<String, String> {
    match chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d_%H:%M:%S") {
        Ok(_) => Ok(s.to_string()),
        Err(e) => Err(format!("날짜 형식 오류: {} (입력: '{}')", e, s)),
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
                    Some(from_json(&x))
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
    //println!("{}", task.creat_time.format("%Y-%m-%d %H:%M:%S"));
    format!(
        "{{\"id\": {}, \"title\": {}, \"creat_time\": {}, \"start_time\": {}, \"finish_time\": {}, \"scheduled_time\": {}, \"deadline\": {}, \"state\": {:?}}}",
        task.id, task.title, task.creat_time.map_or("None".to_string(), |dt| dt.to_string()), task.start_time.map_or("None".to_string(), |dt| dt.to_string()), task.finish_time.map_or("None".to_string(), |dt| dt.to_string()), task.scheduled_time.map_or("None".to_string(), |dt| dt.to_string()), task.deadline_time.map_or("None".to_string(), |dt| dt.to_string()), task.state
    )
}

fn from_json(s: &str) -> Todo {
    let s = s.trim();
    // if !s.starts_with('{') || !s.ends_with('}') {
    //     return None;
    // }

    let mut id = 0;
    let mut title = String::new();
    let mut creat_time: Option<NaiveDateTime> = None;
    let mut start_time: Option<NaiveDateTime> = None;
    let mut finish_time: Option<NaiveDateTime> = None;
    let mut scheduled_time: Option<NaiveDateTime> = None;
    let mut deadline_time: Option<NaiveDateTime> = None;
    let mut state = "Done";
    let state_e: State;

    for todo in s[1..s.len() - 1].split(',') {
        let parts: Vec<_> = todo
            .split(": ")
            .map(|x| x.trim().trim_matches('"'))
            .collect();

        if parts.len() != 2 {
            continue;
        }

        match parts[0] {
            "id" => id = parts[1].parse().unwrap_or(0),
            "title" => title = parts[1].to_string(),
            "creat_time" => creat_time = NaiveDateTime::parse_from_str(parts[1], "%Y-%m-%d %H:%M:%S").ok(),
            "start_time" => start_time = NaiveDateTime::parse_from_str(parts[1], "%Y-%m-%d %H:%M:%S").ok(),
            "finish_time" => finish_time = NaiveDateTime::parse_from_str(parts[1], "%Y-%m-%d %H:%M:%S").ok(),
            "scheduled_time" => scheduled_time = NaiveDateTime::parse_from_str(parts[1], "%Y-%m-%d %H:%M:%S").ok(),
            "deadline" => deadline_time = NaiveDateTime::parse_from_str(parts[1], "%Y-%m-%d %H:%M:%S").ok(),
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

    Todo { id, title, creat_time, start_time, finish_time, scheduled_time, deadline_time, state:state_e }
    
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


