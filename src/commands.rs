use clap::ArgMatches;
use chrono::{NaiveDateTime, NaiveDate, Local};

use crate::store::{save_tasks, show_list, load_tasks};
use crate::{State, Todo};


pub fn commands(matches: ArgMatches) {
    let mut tasks = load_tasks();
    match matches.subcommand() {
        // add 커맨드
        Some(("add", sub_m)) => { 
            let id = 
            if tasks.len() == 0 {
                0
            } else {
                tasks[tasks.len() - 1].id as u32 + 1
            };
            let title = sub_m.get_one::<String>("title").unwrap();

            let none = String::from("None");
            let scheduled_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("scheduled").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let deadline_time = NaiveDateTime::parse_from_str(sub_m.get_one::<String>("deadline").unwrap_or(&none), "%Y-%m-%d_%H:%M:%S").ok();
            let creat_time= NaiveDateTime::parse_from_str(&Local::now().format("%Y-%m-%d_%H:%M:%S").to_string(), "%Y-%m-%d_%H:%M:%S").ok();

            tasks.push(Todo::new(id, title.to_string(), creat_time, scheduled_time, deadline_time));
            save_tasks(tasks);
        }
        // list 커맨드
        Some(("list", sub_m)) => { 
            let done = sub_m.get_flag("done");
            let pending = sub_m.get_flag("pending");
            let ongoing = sub_m.get_flag("ongoing");
            let not_started = sub_m.get_flag("not-started");

            let overdue = sub_m.get_flag("overdue");
            let should_start = sub_m.get_flag("should-start");
            
            let none = String::from("None");
            let scheduled_at = sub_m.get_one::<String>("scheduled-at").map(String::as_str);
            let scheduled_at = match scheduled_at {
                Some("today") => Some(Local::now().date_naive()),
                Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok(),
                None => None,
            };
            let scheduled_before = NaiveDate::parse_from_str(sub_m.get_one::<String>("scheduled-before").unwrap_or(&none), "%Y-%m-%d").ok();
            let scheduled_after = NaiveDate::parse_from_str(sub_m.get_one::<String>("scheduled-after").unwrap_or(&none), "%Y-%m-%d").ok();
            let deadline_at = sub_m.get_one::<String>("deadline-at").map(String::as_str);
            let deadline_at = match deadline_at {
                Some("today") => Some(Local::now().date_naive()),
                Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok(),
                None => None,
            };
            let deadline_before = NaiveDate::parse_from_str(sub_m.get_one::<String>("deadline-before").unwrap_or(&none), "%Y-%m-%d").ok();
            let deadline_after = NaiveDate::parse_from_str(sub_m.get_one::<String>("deadline-after").unwrap_or(&none), "%Y-%m-%d").ok();
            
            let mut state_filters = vec![];
            let mut timeout_filters = false;
            let mut day_filters: Option<NaiveDate> = None;
            let now = NaiveDateTime::parse_from_str(&Local::now().format("%Y-%m-%d_%H:%M:%S").to_string(), "%Y-%m-%d_%H:%M:%S").ok();
            // 테스크 리스트 없는 경우
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                println!("ID.   title.              creat_time.          start_time.          finish_time.         scheduled_time.      deadline.            done.");
                // done 필터 체크
                if done { 
                    state_filters.push(State::Done);
                }
                // pending 필터 체크
                if pending { 
                    state_filters.push(State::Pending);
                }
                // ongoing 필터 체크
                if ongoing { 
                    state_filters.push(State::Ongoing);
                }
                // not_started 필터 체크
                if not_started { 
                    state_filters.push(State::Notstarted);
                }
                // overdue, should_start 필터 체크
                if (overdue, should_start) != (false, false) { 
                    timeout_filters = true;
                }
                // at, before, after 필터 체크
                if (scheduled_at, scheduled_before, scheduled_after, deadline_at, deadline_before, deadline_after) != (None, None, None, None, None, None) {
                    day_filters = Some(Local::now().date_naive());
                }
                // 필터 없는경우
                if state_filters.is_empty() && !timeout_filters && day_filters == None {
                    for task in &tasks {
                        show_list(task);
                    }
                }
                // 필터 있는경우 
                else { 
                    for task in &tasks {
                        // overdue, should 계산
                        let mut overdue_check = false;
                        let mut should_check = false;
                        if timeout_filters {
                            if overdue && task.deadline_time != None && task.deadline_time < now {
                                overdue_check = true;
                            }
                            if should_start && task.scheduled_time != None && task.scheduled_time < now {
                                should_check = true;
                            }
                        }
                        // at, before, after 계산
                        let mut s_at = false;
                        let mut s_before = false;
                        let mut s_after = false;
                        let mut d_at = false;
                        let mut d_before = false;
                        let mut d_after = false;
                        if day_filters != None { 
                            if scheduled_at != None && task.scheduled_time != None && task.scheduled_time.map(|d| d.date()) == scheduled_at {
                                s_at = true;
                            }
                            if scheduled_before != None && task.scheduled_time != None && task.scheduled_time.map(|d| d.date()) <= scheduled_before {
                                s_before = true;
                            }
                            if scheduled_after != None && task.scheduled_time != None && task.scheduled_time.map(|d| d.date()) >= scheduled_after {
                                s_after = true;
                            }
                            if deadline_at != None && task.deadline_time != None && task.deadline_time.map(|d| d.date()) == deadline_at {
                                d_at = true;
                            }
                            if deadline_before != None && task.deadline_time != None && task.deadline_time.map(|d| d.date()) <= deadline_before {
                                d_before = true;
                            }
                            if deadline_after != None && task.deadline_time != None && task.deadline_time.map(|d| d.date()) >= deadline_after {
                                d_after = true;
                            }
                        }
                        // timeout 만 있는경우
                        if state_filters.is_empty() && day_filters == None { 
                            if overdue_check || should_check {
                                show_list(task);
                            }
                        }
                        // state 만 있는경우
                        else if !timeout_filters && day_filters == None {
                            if state_filters.contains(&task.state) {
                                show_list(task);
                            }
                        }
                        // day 만 있는경우
                        else if state_filters.is_empty() && !timeout_filters {
                            if s_at || s_before || s_after || d_at || d_before || d_after {
                                show_list(task);
                            }
                        }
                        // timeout, state 경우
                        else if day_filters == None {
                            if state_filters.contains(&task.state) { 
                                if overdue_check || should_check {
                                    show_list(task);
                                }
                            }
                        }
                        // timeout, day 경우
                        else if state_filters.is_empty() {
                            if overdue_check || should_check { 
                                if s_at || s_before || s_after || d_at || d_before || d_after {
                                    show_list(task);
                                }
                            }
                        }
                        // state, day 경우
                        else if !timeout_filters {
                            if state_filters.contains(&task.state) { 
                                if s_at || s_before || s_after || d_at || d_before || d_after {
                                    show_list(task);
                                }
                            }
                        }
                        // state, timeout, day 경우
                        else {
                            if state_filters.contains(&task.state) { 
                                if s_at || s_before || s_after || d_at || d_before || d_after {
                                    if overdue_check || should_check {
                                        show_list(task);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // edit 커맨드
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
                if let Some(task) = tasks.iter_mut().find(|t|t.id == *id) {
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
        // done 커맨드
        Some(("done", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            let finish_time= NaiveDateTime::parse_from_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), "%Y-%m-%d %H:%M:%S").ok();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id == *id) {
                    task.state = State::Done;
                    task.finish_time = finish_time;
                    if task.start_time.is_none() {
                        task.start_time = finish_time;
                    }
                } else {
                    println!("There is no task with that number.");
                }
                save_tasks(tasks);
            }
        }
        // delete 커맨드
        Some(("delete", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if tasks.iter().any(|t| t.id == *id) {
                    tasks.retain(|t| t.id != *id);
                } else {
                    println!("There is no task with that number.");
                }
                save_tasks(tasks);
            }
        }
        // start 커맨드
        Some(("start", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            let start_time= NaiveDateTime::parse_from_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), "%Y-%m-%d %H:%M:%S").ok();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id == *id) {
                    task.state = State::Ongoing;
                    task.start_time = start_time;
                } else {
                    println!("There is no task with that number.");
                }
                save_tasks(tasks);
            }
        }
        // pending 커맨드
        Some(("pending", sub_m)) => {
            let id = sub_m.get_one::<u32>("id").unwrap();
            if tasks.is_empty() {
                println!("No tasks yet");
            } else {
                if let Some(task) = tasks.iter_mut().find(|t|t.id == *id) {
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