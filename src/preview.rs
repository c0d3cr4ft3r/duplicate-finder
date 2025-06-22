use std::process::Command;

pub fn open_with_system_viewer(path: &str) {
    #[cfg(target_os = "macos")]
    let cmd = {
        if path.ends_with(".jpg") || path.ends_with(".png") || path.ends_with(".mov") || path.ends_with(".mp4") {
            Command::new("qlmanage").args(&["-p", path]).status()
        } else {
            Command::new("open").arg(path).status()
        }
    };

    #[cfg(target_os = "linux")]
    let cmd = Command::new("xdg-open").arg(path).status();

    #[cfg(target_os = "windows")]
    let cmd = Command::new("cmd").args(&["/C", "start", path]).status();

    if let Err(e) = cmd {
        eprintln!("⚠️  Failed to open preview: {}", e);
    }
}
