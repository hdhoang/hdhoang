// -*- compile-command: (concat "rustc --out-dir ~/Dropbox/bin " buffer-file-name) -*-
#![feature(env,process,std_misc)]
use std::env::{set_var, home_dir};
use std::process::Command;
use std::ffi::AsOsStr;

fn main() {
    set_var("DISPLAY", ":0");
    let mut xauth = home_dir().unwrap();
    xauth.push(".Xauthority");
    set_var("XAUTHORITY", xauth.as_os_str());

    let xrandr = Command::new("/usr/bin/xrandr").output().unwrap();
    let output = String::from_utf8_lossy(&xrandr.stdout);
    let orientation = output
        .lines().nth(1).expect("Wrong number of lines")
        .words().nth(3).expect("Wrong number of words");
    let new_screen_orientation = match orientation {
        "inverted" => "normal",
        "left" => "right",
        "right" => "left",
        "(normal" => "inverted",
        _ => unreachable!("Unknown orientation")
    };

    println!("screen rotation {}",
             Command::new("/usr/bin/xrandr")
             .args(&["--output", "LVDS1",
                     "--rotate", new_screen_orientation])
             .status().ok().unwrap());

    let new_touch_orientation = match new_screen_orientation {
        "normal" => "none",
        "left" => "ccw",
        "right" => "cw",
        "inverted" => "half",
        _ => unreachable!("Unknown new orientation")
    };
    println!("touch  rotation {}",
             std::process::Command::new("/usr/bin/xsetwacom")
             .args(&["set", "Atmel Atmel maXTouch Digitizer touch",
                     "Rotate", new_touch_orientation])
             .status().ok().expect("Failed running xsetwacom"))
}
