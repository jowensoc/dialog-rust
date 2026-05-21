use ferris_says::say;
// from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    set_dialog("3", 1);
    set_dialog("2", 1);
    set_dialog("1", 1);
    set_dialog("All Systems Go!", 1);
}

fn set_dialog(dialogtext: &str, delay : u64) {
    if delay > 0 {
        std::thread::sleep(std::time::Duration::from_secs(delay));
    }

    let stdout = stdout();
    let message = String::from(dialogtext);
    let width = message.chars().count();

    print!("\x1Bc");
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
