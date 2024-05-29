//  Day 11: Imagery from the North Pole

// Decked out in his signature red coat, Santa's eyes sparkle brighter than the Northern Star as he navigates through tall shelves packed with newly produced Christmas decorations for the season. Handcrafted glass balls, ornate stars, whimsical snowflakes, festive tinsel - you name it, they have it all.

// Task 1: Served on a silver platter

// The time has come to decorate our surroundings! The elves are getting tired of working with just strings and numbers and bytes, and are in need of some fancy christmas ornaments on the computer screens.

// const DECORATION_IMAGE: _ = include_bytes!("../assets/decoration.png");
use tower_http::services::ServeDir;

///  serve it as a static file so that a GET request to /11/assets/decoration.png responds with the image file and correct headers for MIME type (Content-Type: image/png) and response length (Content-Length: ...).
pub fn task1() -> ServeDir {
    ServeDir::new("assets")
}

use axum::extract::Multipart;

/// Add a POST endpoint /11/red_pixels, that takes in a PNG image in the `image` field of a multipart POST request, and returns the number of pixels in the image that are perceived as "magical red" when viewed with Santa's night vision goggles. The goggles
pub async fn task2(mut mp: Multipart) -> String {
    let mut res = String::new();
    while let Some(field) = mp.next_field().await.unwrap() {
        let name = field.name().unwrap();
        if name != "image" {
            continue;
        } else {
            let body = field.bytes().await.unwrap();
            res = perceive_image(&body).to_string()
        };
    }
    res
}

fn perceive_image(bytes: &[u8]) -> usize {
    use lodepng::decode24;
    let png = decode24(bytes).unwrap();
    png.buffer.iter().filter(is_magical_red).count()
}

use lodepng::RGB;
/// considers a pixel "magical red" if the color values of the pixel fulfill the formula `red > blue + green`.
fn is_magical_red(pixel: &&RGB<u8>) -> bool {
    pixel.r > pixel.b.saturating_add(pixel.g)
}
