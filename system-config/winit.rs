//@ Set up Windows
//!Packing: `$ zip -j winit.zip /mingw32/bin/libgcc_s_dw2-1.dll /mingw32/bin/libwinpthread-1.dll winit.exe`
use std::io::process::Command;

#[cfg(target_os="win32")]
fn main () {
    let cs = ["intl.cpl", "desk.cpl"];
    for c in cs.iter() {
        match Command::new("control").arg(*c).spawn() {
            _ => ()
        }
    }
}
