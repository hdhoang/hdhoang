pub mod grabbers;

use chrono::{DateTime, FixedOffset};
use feed_rs::model::Entry;
use json_feed_model::{Author as JFAuthor, Feed as JFeed, Item as JFItem, Version};
use lol_html::RewriteStrSettings;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Object<'a> {
    title: &'a str,
    // description: Option<&'a str>,
    pub content: Option<&'a str>,
    pub link: &'a str,
    date: DateTime<FixedOffset>,
    authors: Vec<Author<'a>>,
}
#[derive(Debug, Deserialize)]
pub struct Author<'a> {
    pub name: &'a str,
    pub email: Option<&'a str>,
    pub uri: Option<&'a str>,
}

impl<'o> From<Object<'o>> for JFItem {
    fn from(object: Object<'o>) -> Self {
        let mut item = JFItem::new();
        item.set_title(object.title.to_owned());
        item.set_url(object.link.to_owned());
        item.set_id(object.link.to_owned());
        item.set_date_published(object.date.to_rfc3339());

        item.set_authors({
            let mut authors = vec![];
            for person in object.authors {
                let mut a = JFAuthor::new();
                a.set_name((*person.name).to_owned());
                authors.push(a);
            }
            authors
        });

        object.content.map(|s| {
            item.set_content_html(lol_html::rewrite_str(s, crate::create_rewrite_settings()).ok()?)
        });
        item
    }
}

/// Similar to huginn output template:
/// ```json
/// "template": {
///   "title": "Full Text feeds",
///   "link": "https://mysterious-lake-12328.herokuapp.com/scenarios/2",
///   "description": "full text feeds"
///  },
/// ```
pub fn create_output_feed() -> JFeed {
    let mut output = JFeed::new();
    output.set_version(&Version::Version1_1);
    output.set_title("lake feed");
    output.set_feed_url("https://laker.fly.dev/raw/output.json");
    // output.set_home_page_url("https://nest.pijul.com/hdhoang/homes/laker/");
    output
}

/// Convert a `[feed_rs::model::Entry]` to a `[json_feed_model::Item]`, with modified content
///
/// Content modification: see `[create_rewrite_settings]`. Item ID,
/// timestamps, url, authors are copied verbatim. Feed title is
/// prepended to item title.
///
/// Similar to huginn `item` template:
/// ```json
///    "item": {
///      "title": "{{feed.title}}: {{title}}",
///      "description": "{{ content | filters_were_here }} <p> {{ title_text }} <p> {{ extra }}",
///      "link": "{{url}}",
///      "guid": "{{url}}",
///      "pubDate": "{{last_updated}}",
///      "author": "{{authors}}"
///    },
/// ```
/// More about `description` in `[create_rewrite_settings]`
pub fn convert_entry(entry: &Entry, feed_title: &String) -> JFItem {
    let mut item = JFItem::new();

    item.set_id(&entry.id);
    if let Some(ref time) = entry.published {
        item.set_date_published(time.to_rfc3339());
    };
    if let Some(ref time) = entry.updated {
        item.set_date_modified(time.to_rfc3339());
    };

    item.set_title(format!(
        "{feed_title}: {}",
        entry
            .title
            .as_ref()
            .map(|t| t.content.clone())
            .unwrap_or_default()
    ));
    item.set_url(
        entry
            .links
            .get(0)
            .map(|l| l.href.clone())
            .unwrap_or_else(|| "?? no url?".into()),
    );
    item.set_authors({
        let mut authors = vec![];
        for person in &entry.authors {
            let mut a = JFAuthor::new();
            a.set_name(person.name.clone());
            authors.push(a);
        }
        authors
    });

    let new_content = entry
        .summary
        .as_ref()
        .map(|t| t.content.clone())
        .unwrap_or_else(|| "??? no summary html???".into());

    let new_content =
        lol_html::rewrite_str(&new_content, create_rewrite_settings()).expect("rewrote");
    item.set_content_html(new_content);
    item
}

/// Register rewriting settings, to implement both RSS Output's Liquid templating, and perhaps Website Agent extraction.
///
/// Liquid filters on `content`:
/// ```liquid
/// remove: '"mouseover" src=' |
/// remove: 'data-over' |
/// remove: 'height=' |remove: 'width=' |remove: 'resize=' |remove: 'srcset=' |
/// regex_replace: '-[0-9]+x[0-9]+[.]jpg', '.jpg' |regex_replace: '-[0-9]+x[0-9]+[.]png', '.png' |regex_replace: '-[0-9]+x[0-9]+[.]gif', '.gif' |
/// replace: '.png?fit=', '.png?t=' |replace: '.jpg?fit=', '.jpg?t='
/// ```
pub fn create_rewrite_settings() -> RewriteStrSettings<'static, 'static> {
    use lol_html::{element, html_content::ContentType};
    let mut rules = vec![];
    let remove_sizings = element!("img[src]", |img| {
        ["height", "width", "srcset"].map(|attr| img.remove_attribute(attr));
        if let Some(mut unsized_src) = img.get_attribute("src") {
            unsized_src = unsized_src
                .replace("&w", "&_")
                .replace("&h", "&_")
                .replace("-150x150.", ".")
                .replace("?fit", "?_")
                .replace("&resize", "&_");
            img.set_attribute("src", &unsized_src)
                .expect("attr src is always valid")
        }
        Ok(())
    });
    rules.push(remove_sizings);

    let append_title = element!("img[title]", |img| {
        if let Some(title) = img.get_attribute("title") {
            img.after(&format!("<p>{title}</p>"), ContentType::Html)
        }
        Ok(())
    });
    rules.push(append_title);

    let remove_swords_background = element!("a[style]", |a| {
        a.remove_attribute("style");
        Ok(())
    });
    rules.push(remove_swords_background);

    RewriteStrSettings {
        element_content_handlers: rules,
        ..Default::default()
    }
}
