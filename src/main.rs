//use chrono::format;
//use clap::builder::Str;
use clap::{value_parser, Arg, Command};
use dirs::home_dir;
//use std::any::Any;
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
                .arg(Arg::new("scheduled").help("Set deadline time (Y-M-D_H:M:S)").value_parser(verification_time).short('h').long("scheduled"))
                .arg(Arg::new("deadline").help("Set Scheduled start time (Y-M-D_H:M:S)").value_parser(verification_time).short('d').long("deadline"))
        )
        .subcommand(Command::new("list").about("Show all tasks"))
        .subcommand(
            Command::new("edit")
                .about("Edit todo list")
                .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to edit"))
                .arg(Arg::new("title").help("Edit title").short('t').long("title"))
                .arg(Arg::new("creat").help("Edit creat_time").short('c').long("creat"))
                .arg(Arg::new("start").help("Edit start_time").short('s').long("start"))
                .arg(Arg::new("finish").help("Edit finish_time").short('f').long("finish"))
                .arg(Arg::new("scheduled").help("Edit scheduled_time").short('h').long("scheduled"))
                .arg(Arg::new("deadline").help("Edit deadline").short('d').long("deadline"))
        )
        .subcommand(
            Command::new("done")
            .about("completion processing")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to completion"))
        )
        .subcommand(
            Command::new("delete")
            .about("delete processing")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to delete"))
        )
        .subcommand(
            Command::new("start")
            .about("start processing")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to start"))
        )
        .subcommand(
            Command::new("pending")
            .about("pending processing")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to pending"))
        )
        .get_matches();

    //let mut done_check = load_tasks();
    let mut tasks = load_tasks();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let none = String::from("None");
            let id = if tasks.len() < tasks[tasks.len() - 1].id{
                tasks[tasks.len() - 1].id + 1
            } else {
                tasks.len()
            };
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
                    task.id, 
                    task.title, 
                    task.creat_time.map_or("None".to_string(), |dt| dt.to_string()), 
                    task.start_time.map_or("None".to_string(), |dt| dt.to_string()), 
                    task.finish_time.map_or("None".to_string(), |dt| dt.to_string()), 
                    task.scheduled_time.map_or("None".to_string(), |dt| dt.to_string()), 
                    task.deadline_time.map_or("None".to_string(), |dt| dt.to_string()), 
                    task.state);
                }
            }
        }
        Some(("edit", sub_m)) => {
            let none = String::from("None");
            let id = sub_m.get_one::<u32>("id").unwrap();
            let title = sub_m.get_one::<String>("title").unwrap_or(&none);
            let creat = sub_m.get_one::<String>("creat").unwrap_or(&none);
            let creat_time: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&creat, "%Y-%m-%d_%H:%M:%S").ok();
            let start = sub_m.get_one::<String>("start").unwrap_or(&none);
            let start_time: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&start, "%Y-%m-%d_%H:%M:%S").ok();
            let finish = sub_m.get_one::<String>("finish").unwrap_or(&none);
            let finish_time: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&finish, "%Y-%m-%d_%H:%M:%S").ok();           
            let scheduled = sub_m.get_one::<String>("scheduled").unwrap_or(&none);
            let scheduled_time: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&scheduled, "%Y-%m-%d_%H:%M:%S").ok();
            let deadline = sub_m.get_one::<String>("deadline").unwrap_or(&none);
            let deadline_time: Option<NaiveDateTime> = NaiveDateTime::parse_from_str(&deadline, "%Y-%m-%d_%H:%M:%S").ok();

            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id as u32 == *id) {
                    if none != *title {
                        task.title = title.clone();
                    }
                    if none != *creat {
                        task.creat_time = creat_time;
                    }
                    if none != *start {
                        task.start_time = start_time;
                    }
                    if none != *finish {
                        task.finish_time = finish_time;
                    }
                    if none != *scheduled {
                        task.scheduled_time = scheduled_time;
                    }
                    if none != *deadline {
                        task.deadline_time = deadline_time;
                    }
                }
                save_tasks(tasks);
            }
        }
        Some(("done", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id as u32 == *id) {
                    task.state = State::Done;
                }
                save_tasks(tasks);
            }
        }
        Some(("delete", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                tasks.retain(|task| task.id != *id as usize);
                save_tasks(tasks);
            }
        }
        Some(("start", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let start_time= NaiveDateTime::parse_from_str(&now, "%Y-%m-%d %H:%M:%S").ok();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id as u32 == *id) {
                    task.state = State::Ongoing;
                    task.start_time = start_time;
                }
                save_tasks(tasks);
            }
        }
        Some(("pending", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id as u32 == *id) {
                    task.state = State::Pending;
                }
                save_tasks(tasks);
            }
        }

        _ => unreachable!(),
    }
}

// fn done_check(check: Vec<Todo>) -> Vec<Todo> {
//     for d in check {
//         match d.state {
//             State::Done => continue,
//             State::Pending => continue,
//             State::Ongoing => continue,
//             State::Notstarted => todo!(),
//             State::Overdue => todo!(),
//             State::Shouldstart => todo!(),
//         }
//     }
//     check    
// }

fn verification_time(s: &str) -> Result<String, String> {
    match chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d_%H:%M:%S") {
        Ok(_) => Ok(s.to_string()),
        Err(e) => Err(format!("Date format error: {} (You entered: '{}')", e, s)),
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
        task.id, 
        task.title, 
        task.creat_time.map_or("None".to_string(), |dt| dt.to_string()), 
        task.start_time.map_or("None".to_string(), |dt| dt.to_string()), 
        task.finish_time.map_or("None".to_string(), |dt| dt.to_string()), 
        task.scheduled_time.map_or("None".to_string(), |dt| dt.to_string()), 
        task.deadline_time.map_or("None".to_string(), |dt| dt.to_string()), 
        task.state
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


