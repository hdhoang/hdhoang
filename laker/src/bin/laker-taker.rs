use json_feed_model::{Item as JFItem};
use tracing::info;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let stdin = std::io::stdin();
    for line in stdin.lines() {
        let line = line?;
        let object: laker::Object = serde_json::from_str(&line)?;
        info!("input {object:#?}");
        let item: JFItem = object.into();
        info!("output {item:#?}");
    }
    Ok(())
}

// {"title": "Item Example", "description": "This is an example", "content": "Lorem <img title=ipsum> dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.", "link": "https://www.buttersafe.com/2022/09/01/dystopianfuturezoocomics/", "date": "2021-09-08T23:12:05+02:00", "authors": [{"name": "Jane Doe", "email": "jane@example.com", "uri": "https://example.com/author/jane-doe"}]}

// {"title": "Item Example", "description": "This is an example", "content": "Lorem <img title=ipsum> dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.", "link": "https://afistfulofbabies.com/ghost-mirror/", "date": "2021-09-08T23:12:05+02:00", "authors": [{"name": "Jane Doe", "email": "jane@example.com", "uri": "https://example.com/author/jane-doe"}]}
