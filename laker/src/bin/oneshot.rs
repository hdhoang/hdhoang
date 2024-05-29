use feed_rs::parser::parse;
use std::fs::File;

fn main() -> Result<(), anyhow::Error> {
    let mut output = laker::create_output_feed();

    let mut items = vec![];

    for name in &[
        "tests/qwantz-rss-2022-09-02.xml",
        "tests/awkwardyeti-rss-2022-09-03.xml",
        "tests/smbc-rss-2022-09-02.xml",
        "tests/xkcd-atom-2022-09-02.xml",
        "tests/swords-rss-2022-09-03.xml",
        "tests/afistfulofbabies-rss-2022-09-02.xml",
    ] {
        let file = File::open(&name)?;
        let source = parse(file)?;
        let feed_title = source.title.map(|t| t.content).unwrap_or_default();
        for entry in &source.entries {
            items.push(laker::convert_entry(entry, &feed_title));
        }
    }

    output.set_items(items);
    dbg!(&output);
    assert!(output.is_valid(&json_feed_model::Version::Version1_1));
    let out_file = File::create("tests/output.json")?;
    serde_json::to_writer(out_file, &output)?;
    Ok(())
}
