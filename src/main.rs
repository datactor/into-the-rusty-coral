use chrono::{NaiveDateTime};
mod cli; // clap 인터페이스
mod store; // 데이터 저장/로드/패킹/언패킹
mod commands; // clap 커맨드 기능

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    creat_time: Option<NaiveDateTime>,
    start_time: Option<NaiveDateTime>,
    finish_time: Option<NaiveDateTime>,
    scheduled_time: Option<NaiveDateTime>,
    deadline_time: Option<NaiveDateTime>,
    state: State,
}
impl Todo {
    pub fn new(id: u32, title: String, creat_time: Option<NaiveDateTime>, scheduled_time: Option<NaiveDateTime>, deadline: Option<NaiveDateTime>) -> Self {
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
#[derive(Debug)]
#[derive(PartialEq)]
pub enum State {
    Done,
    Pending,
    Ongoing,
    Notstarted,
}

fn main() {
    let matches = cli::clap(); // clap 인터페이스 호출
    commands::commands(matches); // clap 명령어에 따른 동작호출
}