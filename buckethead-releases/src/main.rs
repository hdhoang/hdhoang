use musicbrainz_rs::Error;
use musicbrainz_rs::MusicBrainzClient;
use musicbrainz_rs::{Browse as _, entity::release::Release};

static BUCKETHEAD_MBID: &str = "b5179744-f217-4455-9d8b-17b8d4fdeb93";
static OUR_UA: &str = "buckethead-releases/v0.0.0 ( https://keyoxide.org/aspe:keyoxide.org:UF5U7ORMFX4ELRONPLW22Q7YDE )";

// iterate with curl & jaq:
// static-curl -vsSL --user-agent "$OUR_UA" "http://musicbrainz.org/ws/2/release?fmt=json&artist=$BUCKETHEAD_MBID&type=ep&status=official&inc=recordings&limit=100&offset="$(jaq '(.["release-offset"]|tonumber) + (.releases|length)' prev.json) -o next.json

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = MusicBrainzClient::default();
    client.set_user_agent(OUR_UA).expect("valid UA");

    let mut buckethead_releases = Release::browse();

    buckethead_releases
        .by_artist(BUCKETHEAD_MBID)
        .with_recordings();
    // buckethead_releases.offset = Some(570);
    // buckethead_releases.limit = Some(4);
    dbg!(&buckethead_releases.inner);
    // let mut req = buckethead_releases.as_api_request(&client);
    // req.url.push_str("&type=ep");
    // dbg!(&req.url);
    // dbg!(&req.get_json(&client).await.expect("json"));

    Ok(())
}
