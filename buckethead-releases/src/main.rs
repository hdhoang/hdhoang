use musicbrainz_rs::Error;
use musicbrainz_rs::MusicBrainzClient;
use musicbrainz_rs::{Browse as _, entity::release_group::ReleaseGroup};

static BUCKETHEAD_MBID: &'static str = "b5179744-f217-4455-9d8b-17b8d4fdeb93";
static OUR_UA: &'static str = "buckethead-releases/v0.0.0 ( https://keyoxide.org/aspe:keyoxide.org:UF5U7ORMFX4ELRONPLW22Q7YDE )";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = MusicBrainzClient::default();
    client.set_user_agent(OUR_UA).expect("valid UA");

    let mut buckethead_releases = ReleaseGroup::browse();
    buckethead_releases
        .by_artist(BUCKETHEAD_MBID)
        //        .with_medias()
        .with_url_relations();
    buckethead_releases.offset = Some(570);
    buckethead_releases.limit = Some(4);
    dbg!(&buckethead_releases.inner);
    let mut req = buckethead_releases.as_api_request(&client);
    req.url.push_str("&type=ep");
    dbg!(&req.url);
    dbg!(&req.get_json(&client).await.expect("json"));

    Ok(())
}
