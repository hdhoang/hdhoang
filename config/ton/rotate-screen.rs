fn main() {
    std::os::setenv("DISPLAY", ":0");
    std::os::setenv("XAUTHORITY", "/home/hdhoang/.Xauthority");
    let xrandr = std::io::Command::new("/usr/bin/xrandr");

    let query = xrandr.clone();
    let qo = query.output().ok().expect("Failed running xrandr").output;
    let orientation = std::str::from_utf8(qo.as_slice()).ok().expect("Output is invalid")
        .lines().nth(1).expect("Wrong number of lines")
        .words().nth(3).expect("Wrong number of words");
    let new_screen_orientation = match orientation {
        "inverted" => "normal",
        "left" => "right",
        "right" => "left",
        "(normal" => "inverted",
        _ => unreachable!("Unknown orientation")
    };

    println!("screen rotation {}", xrandr.clone()
             .args(&["--output", "LVDS1", "--rotate", new_screen_orientation])
             .status().ok().unwrap());

    let new_touch_orientation = match new_screen_orientation {
        "normal" => "none",
        "left" => "ccw",
        "right" => "cw",
        "inverted" => "half",
        _ => unreachable!("Unknown new orientation")
    };
    println!("touch  rotation {}", std::io::Command::new("/usr/bin/xsetwacom")
             .args(&["set", "Atmel Atmel maXTouch Digitizer touch",
                     "Rotate", new_touch_orientation])
             .status().ok().expect("Failed running xsetwacom"))
}
