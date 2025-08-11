use clap::{ArgAction, Arg, Command, ArgMatches, value_parser};
use crate::store::verification_time;

pub fn clap() -> ArgMatches {
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
                .arg(Arg::new("ongoing").help("Show ongoing tasks").short('o').long("ongoing").action(ArgAction::SetTrue))
                .arg(Arg::new("not-started").help("Show not-started tasks").short('n').long("not-started").action(ArgAction::SetTrue))
                .arg(Arg::new("overdue").help("Show tasks past their deadline").long("overdue").action(ArgAction::SetTrue))
                .arg(Arg::new("should-start").help("Show tasks that have passed their scheduled start date but have not yet started").long("should-start").action(ArgAction::SetTrue))
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
                .arg(Arg::new("creat").help("Edit creat_time").value_parser(verification_time).short('c').long("creat"))
                .arg(Arg::new("start").help("Edit start_time").value_parser(verification_time).short('s').long("start"))
                .arg(Arg::new("finish").help("Edit finish_time").value_parser(verification_time).short('f').long("finish"))
                .arg(Arg::new("scheduled").help("Edit scheduled_time").value_parser(verification_time).short('h').long("scheduled"))
                .arg(Arg::new("deadline").help("Edit deadline").value_parser(verification_time).short('d').long("deadline"))
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
    matches
}