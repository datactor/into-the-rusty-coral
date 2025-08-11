use dirs::home_dir;
use std::fs;
use std::path::PathBuf;
use chrono::NaiveDateTime;
use crate::{State, Todo};

// 날짜_시간 입력 검사 (%Y-%m-%d_%H:%M:%S)
pub fn verification_time(s: &str) -> Result<String, String> {
    match chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d_%H:%M:%S") {
        Ok(_) => Ok(s.to_string()),
        Err(e) => Err(format!("Date format error: {} (You entered: '{}')", e, s)),
    }
}
// 리스트 화면 출력
pub fn show_list(task: &Todo) {
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
// json 파일 저장 및 경로불러오기
pub fn get_data_file() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push("todo.json");
    path
}
// json 파일 데이터 병렬화
pub fn load_tasks() -> Vec<Todo> {
    let path = get_data_file();
    if path.exists() {
        let data = fs::read_to_string(path).unwrap();
        let data = data
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'));

        if let Some(s) = data {
            s.split("}, {")
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
// json 파일로 데이터 직렬화
pub fn save_tasks(tasks: Vec<Todo>) {
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
// 데이터 헤더정보 입력 및 언패킹
pub fn to_json(task: &Todo) -> String {
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
// 헤더정보 토대로 데이터 패킹
pub fn from_json(s: &str) -> Todo {
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
            "id" => id = parts[1].parse::<u32>().unwrap_or(0),
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
            _ => state_e = State::Done,
    };

    Todo { id, title, creat_time, start_time, finish_time, scheduled_time, deadline_time, state:state_e }
    
}
