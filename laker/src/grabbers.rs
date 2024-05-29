use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Fragments {
    pub main: String,
    pub extra: Option<String>,
}

pub fn grab(
    main_selector: &str,
    extra_selector: Option<String>,
    html: &str,
) -> Result<Fragments, anyhow::Error> {
    let html = Html::parse_fragment(html);

    let main_selector = Selector::parse(main_selector).expect("valid selector");
    let main = html
        .select(&main_selector)
        .next()
        .expect("main match")
        .html();

    let mut extra = None;
    if let Some(extra_selector) = extra_selector {
        let extra_selector = Selector::parse(&extra_selector).expect("valid selector");
        extra = html.select(&extra_selector).next().map(|e| e.html());
    };

    Ok(Fragments { main, extra })
}
