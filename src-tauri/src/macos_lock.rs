use std::process::Command;

pub fn lock_screen() -> Result<(), String> {
    let output = Command::new("CGSession")
        .arg("-suspend")
        .output();

    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to lock screen: {}", e))
    }
}
