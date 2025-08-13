use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub creat_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub finish_time: Option<NaiveDateTime>,
    pub scheduled_time: Option<NaiveDateTime>,
    pub deadline_time: Option<NaiveDateTime>,
    pub state: State,
}

impl Todo {
    pub fn new(
        id: u32,
        title: String,
        create_time: Option<NaiveDateTime>,
        scheduled_time: Option<NaiveDateTime>,
        deadline: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            title,
            creat_time: create_time,
            start_time: None,
            finish_time: None,
            scheduled_time,
            deadline_time: deadline,
            state: State::Notstarted,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Done,
    Pending,
    Ongoing,
    Notstarted,
}
