use std::process::Command;

pub fn focus_existing() -> Result<(), ()> {
    let try_wmctrl = Command::new("wmctrl").args(["-a", "ChatGPT"]).status();
    if let Ok(s) = try_wmctrl {
        if s.success() { return Ok(()); }
    }
    let try_xdotool = Command::new("xdotool").args(["search", "--name", "ChatGPT", "windowactivate"]).status();
    if let Ok(s) = try_xdotool {
        if s.success() { return Ok(()); }
    }
    Err(())
}
