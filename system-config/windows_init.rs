use std::io::process::{Command,ProcessOutput};

#[cfg(target_os="win32")]
fn main () {
    match Command::new("control").arg("desk.cpl").spawn() {
        Err(e) => println!("failed spawning desk.cpl: {}", e),
        _ => ()
    }
    match Command::new("reg")
        .args(["add",r#"HKCU\Keyboard Layout\Preload"#
               ,"/v","1"
               ,"/t","REG_SZ"
               // US-Dvorak
               ,"/d","00010409"
               // forcibly
               ,"/f"]).output() {
            Err(e) => println!("failed setting keyboard layout: {}", e),
            Ok(ProcessOutput{error:e, output: o, status: s}) => {
                let useful = if s.success() {o} else {e};
                println!("{}", String::from_utf8_lossy(useful.as_slice()))
            }
        }
}
