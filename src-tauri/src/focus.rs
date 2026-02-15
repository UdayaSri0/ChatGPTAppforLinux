use std::process::Command;

pub fn focus_existing() -> Result<(), ()> {
    for title in ["ChatGPT Native", "ChatGPT Shell", "ChatGPT"] {
        let try_wmctrl = Command::new("wmctrl").args(["-a", title]).status();
        if let Ok(s) = try_wmctrl {
            if s.success() {
                return Ok(());
            }
        }

        let try_xdotool = Command::new("xdotool")
            .args(["search", "--name", title, "windowactivate"])
            .status();
        if let Ok(s) = try_xdotool {
            if s.success() {
                return Ok(());
            }
        }
    }
    Err(())
}
