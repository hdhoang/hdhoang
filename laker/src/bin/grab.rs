use json_feed_model::Item as JFItem;
use laker::grabbers::grab;
use std::time::Duration;
use tracing::info;
use ureq::AgentBuilder;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args().skip(1);
    let main = args.next().expect("main selector");
    let extra = args.next();

    let agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    let stdin = std::io::stdin();
    for line in stdin.lines() {
        let line = line?;
        let mut object: laker::Object = serde_json::from_str(&line)?;
        info!("input {object:#?}");
        let html = agent.get(&object.link).call()?.into_string()?;
        let laker::grabbers::Fragments { mut main, extra } = grab(&main, extra.clone(), &html)?;
        if let Some(e) = extra {
            main.push_str(&e)
        }
        object.content = Some(main.as_ref());
        let item: JFItem = object.into();
        info!("input {item:#?}");
    }

    Ok(())
}
