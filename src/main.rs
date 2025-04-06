use prepare_commit_msg::hooks::run_hook;
use std::{env, error::Error};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("commit-msg-hook: missing message file argument.".into());
    }
    let message_file = &args[1];
    run_hook(message_file, ".")
}
