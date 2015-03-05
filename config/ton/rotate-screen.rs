// -*- compile-command: "rustc --out-dir ~/Dropbox/bin rotate-screen.rs" -*-
#![feature(env,process)]
use std::env;
use std::process::Command;

fn main() {
    env::set_var("DISPLAY", ":0");
    env::set_var("XAUTHORITY", "/home/hdhoang/.Xauthority");

    let xrandr = Command::new("/usr/bin/xrandr").output().unwrap();
    let output = String::from_utf8_lossy(&xrandr.stdout);
    let orientation = output
        .lines().nth(1).expect("Wrong number of lines")
        .words().nth(3).expect("Wrong number of words");
    let new_screen_orientation = match orientation {
        "(normal" => "left",
        "left" => "inverted",
        "inverted" => "right",
        "right" => "normal",
        _ => unreachable!("Unknown orientation")
    };

    println!("screen rotation {}",
             Command::new("/usr/bin/xrandr")
             .args(&["--output", "LVDS1",
                     "--rotate", new_screen_orientation])
             .status().ok().unwrap());

    let new_transformation_matrix = match new_screen_orientation {
        "normal"   => ["1","0","0",
                       "0","1","0",
                       "0","0","1"],
        "left"     => ["0","1","0",
                       "-1","0","1",
                       "0","0","1"],
        "inverted" => ["-1","0","1",
                       "0","-1","1",
                       "0","0","1"],
        "right"    => ["0","-1","1",
                       "1","0","0",
                       "0","0","1"],
        _ => unreachable!("Unknown orientation")
    };
    for device in &["SynPS/2 Synaptics TouchPad", "TPPS/2 IBM TrackPoint"] {
        println!("{} rotation {}", device,
                 Command::new("/usr/bin/xinput").arg("set-prop")
                 .arg(device).arg("Coordinate Transformation Matrix")
                 .args(&new_transformation_matrix)
                 .status().ok().expect("Failed running xinput"))
    };

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
