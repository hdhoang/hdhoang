fn main() {
    std::os::setenv("DISPLAY", ":0");
    std::os::setenv("XAUTHORITY", "/home/hdhoang/.Xauthority");
    let xrandr = std::io::Command::new("/usr/bin/xrandr");

    let query = xrandr.clone();
    let qo = query.output().ok().expect("No output").output;
    let orientation = std::str::from_utf8(qo.as_slice()).expect("No output")
        .lines().nth(1).expect("Wrong number of lines")
        .words().nth(3).expect("Wrong number of words");
    let new_orientation = match orientation {
        "inverted" => "normal",
        "left" => "right",
        "right" => "left",
        _ => "inverted",
    };

    println!("rotate {}", xrandr.clone().args(&["--output", "LVDS1", "--rotate", new_orientation])
             .status().ok().unwrap())
}
