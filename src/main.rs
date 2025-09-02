use anyhow::Result;
use rustyline::{Editor, error::ReadlineError};
use std::process::{Command, Stdio};
use std::io::{self, Write};
use rustyline::history::DefaultHistory;
use chrono::Local;

fn main() -> Result<()> {
    println!("🔹 Rust Shell — type 'exit' to quit.");

    // Line editor for input (with history support)
    let mut rl = Editor::<(), DefaultHistory>::new()?;

    loop {
        let prompt = "\x1b[32mrs-shell> \x1b[0m"; // green text
        let readline = rl.readline(prompt);
        match readline {
    Ok(line) => {
        let trimmed = line.trim();

        // Save command into history
        rl.add_history_entry(trimmed)?;

        if trimmed.eq_ignore_ascii_case("exit") {
            println!("👋 Bye!");
            break;
        }

        // 🔹 custom command: show today's date
        if trimmed.eq_ignore_ascii_case("date") {
            let today = chrono::Local::now().format("%A, %d %B %Y").to_string();
            println!("📅 Today is {}", today);
            continue; // go to next prompt
        }

        if trimmed.eq_ignore_ascii_case("time") {
            let now = Local::now().format("%H:%M:%S").to_string();
            println!("⏰ Current time is {}", now);
            continue;
}
if trimmed.eq_ignore_ascii_case("clear") {
    // Clear the terminal screen
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    continue;
} 

        // Otherwise run system command...
        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(["-NoProfile", "-Command", trimmed])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(trimmed)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        };

        // (handle output here…)
    }

    Err(ReadlineError::Interrupted) => { /* ... */ }
    Err(ReadlineError::Eof) => { /* ... */ }
    Err(err) => { /* ... */ }
}
    }

    Ok(())
}
