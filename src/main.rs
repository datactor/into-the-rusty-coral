//use chrono::format;
//use clap::builder::Str;
//use clap::{value_parser, Arg, ArgAction, Command};
use clap::*;
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
#[derive(PartialEq)]
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
        .subcommand(
            Command::new("list")
                .about("Show tasks")
                .arg(Arg::new("done").help("Show done tasks").short('d').long("done").action(ArgAction::SetTrue))
                .arg(Arg::new("pending").help("Show pending tasks").short('p').long("pending").action(ArgAction::SetTrue))
                .arg(Arg::new("ongoing").help("Show ongoing tasks").short('g').long("ongoing").action(ArgAction::SetTrue))
                .arg(Arg::new("not-started").help("Show not-started tasks").short('n').long("not-started").action(ArgAction::SetTrue))
                .arg(Arg::new("overdue").help("Show tasks past their deadline").short('o').long("overdue").action(ArgAction::SetTrue))
                .arg(Arg::new("should-start").help("Show tasks that have passed their scheduled start date but have not yet started").short('s').long("should-start").action(ArgAction::SetTrue))
                .arg(Arg::new("scheduled-at").help("Show scheduled tasks for a specific date").long("scheduled-at"))
                .arg(Arg::new("scheduled-before").help("Show tasks scheduled before a specific date").long("scheduled-before"))
                .arg(Arg::new("scheduled-after").help("Show tasks scheduled after a specific date").long("scheduled-after"))
                .arg(Arg::new("deadline-at").help("Show tasks due on a specific date").long("deadline-at"))
                .arg(Arg::new("deadline-after").help("Show tasks due after a specific date").long("deadline-after"))
                .arg(Arg::new("deadline-before").help("Show tasks due before a specific date").long("deadline-before"))
        )
        .subcommand(
            Command::new("edit")
                .about("Edit todo task")
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
            .about("completion task")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to completion"))
        )
        .subcommand(
            Command::new("delete")
            .about("delete task")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to delete"))
        )
        .subcommand(
            Command::new("start")
            .about("start task")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to start"))
        )
        .subcommand(
            Command::new("pending")
            .about("pending task")
            .arg(Arg::new("id").required(true).value_parser(value_parser!(u32)).help("ID to pending"))
        )
        .get_matches();

    let mut tasks = load_tasks();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let id = if tasks.len() < tasks[tasks.len() - 1].id{
                tasks[tasks.len() - 1].id + 1
            } else {
                tasks.len()
            };
            let title = sub_m.get_one::<String>("title").unwrap();

            let none = String::from("None");
            let scheduled_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("scheduled").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let deadline_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("deadline").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let creat_time= NaiveDateTime::parse_from_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), "%Y-%m-%d %H:%M:%S").ok();

            tasks.push(Todo::new(id, title.to_string(), creat_time, scheduled_time, deadline_time));
            save_tasks(tasks);
        }
        Some(("list", sub_m)) => {
            let mut filters = vec![];
            let done = sub_m.get_flag("done");
            let pending = sub_m.get_flag("pending");
            let ongoing = sub_m.get_flag("ongoing");
            let not_started = sub_m.get_flag("not-started");

            let overdue = sub_m.get_flag("overdue");
            let should_start = sub_m.get_flag("should-start");
            
            let none = String::from("None");
            let scheduled_at = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("scheduled-at").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let scheduled_before = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("scheduled-before").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let scheduled_after = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("scheduled-after").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let deadline_at = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("deadline-at").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let deadline_before = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("deadline-before").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let deadline_after = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("deadline-after").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();

            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                println!("ID.   title.              creat_time.          start_time.          finish_time.         scheduled_time.      deadline.            done.");
                if done {
                    filters.push(State::Done);
                }
                if pending {
                    filters.push(State::Pending);
                }
                if ongoing {
                    filters.push(State::Ongoing);
                }
                if not_started {
                    filters.push(State::Notstarted);
                }

                if filters.is_empty() {
                    for task in &tasks {
                        show_list(task);
                    }
                } else {
                    for task in &tasks {
                        if filters.contains(&task.state) {
                            show_list(task);
                        }
                    }
                }
            }
        }
        Some(("edit", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();

            let none = String::from("None");
            let title = sub_m.get_one::<String>("title").unwrap_or(&none);
            let creat_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("creat").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let start_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("start").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let finish_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("finish").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();           
            let scheduled_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("scheduled").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let deadline_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("deadline").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();

            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id as u32 == *id) {
                    if none != *title {
                        task.title = title.clone();
                    } if let Some(t) = creat_time {
                        task.creat_time = Some(t);
                    } if let Some(t) = start_time {
                        task.start_time = Some(t);
                    } if let Some(t) = finish_time {
                        task.finish_time = Some(t);
                    } if let Some(t) = scheduled_time {
                        task.scheduled_time = Some(t);
                    } if let Some(t) = deadline_time {
                        task.deadline_time = Some(t);
                    }
                } else {
                    println!("There is no task with that number.");
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
                } else {
                    println!("There is no task with that number.");
                }
                save_tasks(tasks);
            }
        }
        Some(("delete", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if tasks.iter().any(|t| t.id as u32 == *id) {
                    tasks.retain(|task| task.id != *id as usize);
                } else {
                    println!("There is no task with that number.");
                }
                save_tasks(tasks);
            }
        }
        Some(("start", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            let start_time= NaiveDateTime::parse_from_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), "%Y-%m-%d %H:%M:%S").ok();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id as u32 == *id) {
                    task.state = State::Ongoing;
                    task.start_time = start_time;
                } else {
                    println!("There is no task with that number.");
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
                } else {
                    println!("There is no task with that number.");
                }
                save_tasks(tasks);
            }
        }

        _ => unreachable!(),
    }
}

fn show_list(task: &Todo) {
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


