use std::io::process::Command;

#[cfg(target_os="win32")]
fn main () {
    let cs = ["intl.cpl", "desk.cpl"];
    for c in cs.iter() {
        match Command::new("control").arg(c.to_string()).spawn() {
            _ => ()
        }
    }
}
