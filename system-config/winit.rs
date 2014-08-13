//@ Set up Windows
//!Packing: `$ zip -j winit.zip /mingw32/bin/libgcc_s_dw2-1.dll /mingw32/bin/libwinpthread-1.dll winit.exe caps2ctrl.exe`
use std::io::process::Command;

fn run(args: &[&str]) -> () {
    let p = match Command::new(args[0])
        .args(args.tail()).spawn() {
            Ok(child) => child,
            Err(e) => fail!(e)
        };
    p.forget()
}

fn main () {
    run(["control", "desk.cpl"]);
    run(["rundll32", "shell32.dll,Control_RunDLL", "input.dll"]);
    run(["caps2ctrl"]);
}
