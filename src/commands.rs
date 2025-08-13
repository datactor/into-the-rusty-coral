use chrono::NaiveDate;
use clap::ArgMatches;

use crate::model::{State, Todo};
use crate::store::{save_tasks, show_list};
use crate::utils::{get_id, next_id, now, parse_day_opt, parse_dt_opt};

pub fn cmd_add(sub_m: &ArgMatches, tasks: &mut Vec<Todo>) {
    let id = next_id(tasks);
    let title = sub_m.get_one::<String>("title").unwrap();

    let scheduled_time = parse_dt_opt(sub_m.get_one::<String>("scheduled"));
    let deadline_time = parse_dt_opt(sub_m.get_one::<String>("deadline"));
    let create_time = Some(now());

    tasks.push(Todo::new(
        id,
        title.to_string(),
        create_time,
        scheduled_time,
        deadline_time,
    ));

    save_tasks(tasks);
}

pub fn cmd_list(sub_m: &ArgMatches, tasks: &[Todo]) {
    if tasks.is_empty() {
        println!("No tasks yet");
        return;
    }

    let mut state_filters: Vec<State> = Vec::new();

    if sub_m.get_flag("done") {
        state_filters.push(State::Done);
    }
    if sub_m.get_flag("pending") {
        state_filters.push(State::Pending);
    }
    if sub_m.get_flag("ongoing") {
        state_filters.push(State::Ongoing);
    }
    if sub_m.get_flag("not-started") {
        state_filters.push(State::Notstarted);
    }

    let overdue = sub_m.get_flag("overdue");
    let should_start = sub_m.get_flag("should-start");

    let scheduled_at = parse_day_opt(sub_m.get_one::<String>("scheduled-at"));
    let scheduled_before = parse_day_opt(sub_m.get_one::<String>("scheduled-before"));
    let scheduled_after = parse_day_opt(sub_m.get_one::<String>("scheduled-after"));
    let deadline_at = sub_m.get_one::<String>("deadline-at").map(String::as_str);
    let deadline_before = parse_day_opt(sub_m.get_one::<String>("deadline-before"));
    let deadline_after = parse_day_opt(sub_m.get_one::<String>("deadline-after"));

    let now = now();

    println!("ID.   title.              creat_time.          start_time.          finish_time.         scheduled_time.      deadline.            done.");

    for task in tasks {
        let state_ok = if state_filters.is_empty() {
            true
        } else {
            state_filters.contains(&task.state)
        };

        let overdue_ok = if overdue {
            task.deadline_time.map(|d| d < now).unwrap_or(false)
        } else {
            true
        };

        let should_start_ok = if should_start {
            task.scheduled_time
                .map(|s| s < now && matches!(task.state, State::Notstarted))
                .unwrap_or(false)
        } else {
            true
        };

        let sched_day = task.scheduled_time.map(|x| x.date());
        let deadl_day = task.deadline_time.map(|x| x.date());

        let sched_ok = scheduled_at.map(|d| sched_day == Some(d)).unwrap_or(true)
            && scheduled_before
                .map(|d| sched_day.map(|s| s <= d).unwrap_or(false))
                .unwrap_or(true)
            && scheduled_after
                .map(|d| sched_day.map(|s| s >= d).unwrap_or(false))
                .unwrap_or(true);

        let deadl_ok = deadline_at
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .map(|d| deadl_day == Some(d))
            .unwrap_or(true)
            && deadline_before
                .map(|d| deadl_day.map(|s| s <= d).unwrap_or(false))
                .unwrap_or(true)
            && deadline_after
                .map(|d| deadl_day.map(|s| s >= d).unwrap_or(false))
                .unwrap_or(true);

        if state_ok && overdue_ok && should_start_ok && sched_ok && deadl_ok {
            show_list(task);
        }
    }
}

pub fn cmd_edit(sub_m: &ArgMatches, tasks: &mut Vec<Todo>) {
    if tasks.is_empty() {
        println!("No tasks yet");
        return;
    }

    let id = get_id(sub_m);
    let title = sub_m.get_one::<String>("title");
    let created_time = parse_dt_opt(sub_m.get_one::<String>("creat"));
    let start_time = parse_dt_opt(sub_m.get_one::<String>("start"));
    let finish_time = parse_dt_opt(sub_m.get_one::<String>("finish"));
    let scheduled = parse_dt_opt(sub_m.get_one::<String>("scheduled"));
    let deadline = parse_dt_opt(sub_m.get_one::<String>("deadline"));

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id.unwrap_or(0)) {
        if let Some(t) = title {
            task.title = t.clone();
        }
        if let Some(t) = created_time {
            task.creat_time = Some(t);
        }
        if let Some(t) = start_time {
            task.start_time = Some(t);
        }
        if let Some(t) = finish_time {
            task.finish_time = Some(t);
        }
        if let Some(t) = scheduled {
            task.scheduled_time = Some(t);
        }
        if let Some(t) = deadline {
            task.deadline_time = Some(t);
        }

        save_tasks(tasks);
    } else {
        println!("There is no task with that number.");
    }
}

pub fn cmd_done(sub_m: &ArgMatches, tasks: &mut Vec<Todo>) {
    if tasks.is_empty() {
        println!("No tasks yet");
        return;
    }

    let id = get_id(sub_m).unwrap();
    let now = now();

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.state = State::Done;
        task.finish_time = Some(now);
        if task.start_time.is_none() {
            task.start_time = Some(now);
        }
    } else {
        println!("There is no task with that number.");
    }

    save_tasks(tasks);
}

pub fn cmd_delete(sub_m: &ArgMatches, tasks: &mut Vec<Todo>) {
    if tasks.is_empty() {
        println!("No tasks yet");
        return;
    }

    let id = get_id(sub_m).unwrap();
    tasks.retain(|t| t.id != id);

    save_tasks(tasks);
}

pub fn cmd_start(sub_m: &ArgMatches, tasks: &mut Vec<Todo>) {
    if tasks.is_empty() {
        println!("No tasks yet");
        return;
    }

    let id = get_id(sub_m).unwrap();

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.state = State::Ongoing;
        task.start_time = Some(now());

        save_tasks(tasks);
    } else {
        println!("There is no task with that number.");
    }
}

pub fn cmd_pending(sub_m: &ArgMatches, tasks: &mut Vec<Todo>) {
    if tasks.is_empty() {
        println!("No tasks yet");
        return;
    }

    let id = get_id(sub_m).unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.state = State::Pending;
        save_tasks(tasks);
    } else {
        println!("There is no task with that number.");
    }
}
