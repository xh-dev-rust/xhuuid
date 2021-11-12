use uuid::Uuid;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args:Vec<String> = env::args().collect();
    let my_uuid = Uuid::new_v4();
    if args.len() == 2 {
        if "-p" == &args[1] || "--print" == &args[1] {
            println!("{}", my_uuid);
        }
        else if "-h" == &args[1] || "--help" == &args[1] {
            println!("xhuuid [-p|--print] [-h|--help]");
            exit(0)
        }
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let rs = ctx.set_contents(my_uuid.to_string());
    return rs
}