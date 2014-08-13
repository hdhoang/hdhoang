//@ Set up Windows
//!Packing: `$ zip -j winit.zip /mingw32/bin/libgcc_s_dw2-1.dll /mingw32/bin/libwinpthread-1.dll winit.exe caps2ctrl.exe`
use std::io::process::Command;

#[cfg(target_os="win32")]
fn open_cpl(cpl: &str) -> () {
    match Command::new("control")
        .arg(cpl.into_string().append(".cpl")).spawn() {
            Err(e) => { println!("{}", e) },
            Ok(_) => { () }
        }
}

fn main () {
    for c in ["intl", "desk"].iter() {
        open_cpl(*c);
    }
    match Command::new("caps2ctrl").spawn() { _ => () }
}
