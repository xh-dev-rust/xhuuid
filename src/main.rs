use uuid::Uuid;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let my_uuid = Uuid::new_v4();
    let rs = ctx.set_contents(my_uuid.to_string());
    return rs
    // println!("{}", my_uuid);
    // Ok(())
}