use crate::commands::{cmd_add, cmd_delete, cmd_done, cmd_edit, cmd_list, cmd_pending, cmd_start};

mod cli; // clap 인터페이스
mod commands;
mod model;
mod store;
mod utils; // 데이터 저장/로드/패킹/언패킹 // clap 커맨드 기능 // Todo 모델

fn main() {
    let matches = cli::build_cli().get_matches();

    let mut tasks = store::load_tasks();

    match matches.subcommand() {
        Some(("add", sub_m)) => cmd_add(sub_m, &mut tasks),
        Some(("list", sub_m)) => cmd_list(sub_m, &tasks),
        Some(("edit", sub_m)) => cmd_edit(sub_m, &mut tasks),
        Some(("done", sub_m)) => cmd_done(sub_m, &mut tasks),
        Some(("delete", sub_m)) => cmd_delete(sub_m, &mut tasks),
        Some(("start", sub_m)) => cmd_start(sub_m, &mut tasks),
        Some(("pending", sub_m)) => cmd_pending(sub_m, &mut tasks),
        _ => unreachable!(),
    }
}
