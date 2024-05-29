use axum::{
    extract::{self, Path},
    routing::{get, post},
    Router,
};

mod day11;
mod day21;
mod day6;
mod day8;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let mut router = Router::new()
        .route("/", get(ok))
        .route("/-1/error", get(err))
        .route("/1/*num", get(basen))
        .route("/4/strength", post(day4))
        .route("/4/contest", post(day4_contest))
        .route("/6", post(day6::runner))
        .route("/8/drop/:pokedex", get(day8::drop))
        .route("/8/weight/:pokedex", get(day8::weight))
        .route("/11/red_pixels", post(day11::task2))
        .nest_service("/11/assets", day11::task1());
    router = router
        .route("/21/coords/:binary", get(day21::coords))
        .route("/21/country/:binary", get(day21::country));

    Ok(router.into())
}

#[derive(serde::Deserialize)]
struct Reindeer {
    name: String,
    strength: u8,

    speed: f32,
    height: u8,
    antler_width: u8,
    snow_magic_power: u16,
    // #[serde(rename=(]
    // favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: u8,
}

#[derive(serde::Serialize, Default)]
struct Winners {
    fastest: String,
    tallest: String,
    #[serde(rename = "magician")]
    magic: String,
    consumer: String,
}
async fn day4_contest(extract::Json(rs): extract::Json<Vec<Reindeer>>) -> axum::Json<Winners> {
    let mut winners = Winners::default();
    let fastest = rs
        .iter()
        .max_by(|r, or| r.speed.total_cmp(&or.speed))
        .unwrap();
    winners.fastest = format!(
        "Speeding past the finish line with a strength of {0} is {1}",
        fastest.strength, fastest.name
    );

    let tallest = rs.iter().max_by_key(|r| r.height).unwrap();
    winners.tallest = format!(
        "{0} is standing tall with his {1} cm wide antlers",
        tallest.name, tallest.antler_width
    );

    let magic = rs.iter().max_by_key(|r| r.snow_magic_power).unwrap();
    winners.magic = format!(
        "{0} could blast you away with a snow magic power of {1}",
        magic.name, magic.snow_magic_power
    );
    let consumer = rs.iter().max_by_key(|r| r.candies).unwrap();
    winners.consumer = format!(
        "{0} ate lots of candies, but also some grass",
        consumer.name
    );

    axum::Json(winners)
}

async fn day4(extract::Json(rs): extract::Json<Vec<Reindeer>>) -> String {
    rs.iter().map(|r| r.strength).sum::<u8>().to_string()
}
fn pow3(n: i32) -> String {
    (n * n * n).to_string()
}

async fn basen(Path(p): Path<String>) -> String {
    let mut nums = vec![];
    for segment in p.split('/') {
        nums.push(segment.parse::<i32>().unwrap())
    }
    let n = nums.into_iter().reduce(|a, b| a ^ b).unwrap();
    pow3(n)
}

async fn ok() -> &'static str {
    "eh"
}

async fn err() -> axum::http::StatusCode {
    axum::http::StatusCode::INTERNAL_SERVER_ERROR
}
