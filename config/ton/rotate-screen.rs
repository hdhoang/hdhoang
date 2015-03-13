// -*- compile-command: "rustc --out-dir ~/Dropbox/bin rotate-screen.rs" -*-
#![feature(env,process)]
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<_> = env::args().collect();
    let new_screen_orientation: &str;

    if args.len() == 2 {
        new_screen_orientation = &args[1];
    } else {
        let xrandr = Command::new("/usr/bin/xrandr").output().unwrap();
        let output = String::from_utf8_lossy(&xrandr.stdout);
        let orientation = output
            .lines().nth(1).expect("Wrong number of lines")
            .words().nth(3).expect("Wrong number of words");
        new_screen_orientation = match orientation {
            "(normal" => "left",
            "left" => "inverted",
            "inverted" => "right",
            "right" => "normal",
            _ => unreachable!("Unknown orientation {}.", orientation)
        };
    }

    println!("screen rotation {}",
             Command::new("/usr/bin/xrandr")
             .args(&["--output", "LVDS1",
                     "--rotate", new_screen_orientation])
             .status().ok().unwrap());

    let new_transformation_matrix = match new_screen_orientation {
        "normal"   => ["1","0","0",
                       "0","1","0",
                       "0","0","1"],
        "left"     => ["0","-1","1",
                       "1","0","0",
                       "0","0","1"],
        "inverted" => ["-1","0","1",
                       "0","-1","1",
                       "0","0","1"],
        "right"    => ["0","1","0",
                       "-1","0","1",
                       "0","0","1"],
        _ => unreachable!("Unknown orientation {}.", new_screen_orientation)
    };
    for device in &["SynPS/2 Synaptics TouchPad", "TPPS/2 IBM TrackPoint", "Atmel Atmel maXTouch Digitizer touch"] {
        println!("{} rotation {}", device,
                 Command::new("/usr/bin/xinput").arg("set-prop")
                 .arg(device).arg("Coordinate Transformation Matrix")
                 .args(&new_transformation_matrix)
                 .status().ok().expect("Failed running xinput"))
    };
}
