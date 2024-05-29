# What i want to copy out of huginn

## Memorials for the goners

- Mark Pilgrim
- happle tea
- abstruse goose
- 909sickle
- channelate

## Flows

i use these agents on huginn:

```mermaid
C[Time trigger]-->RA[RSS Agents]-->ST
RA-->G
G[Grabbers website agents]-->ST
ST[show title text]-->FT
FT[RSS Output Agents]
```

There's no secret besides the feed-fetching token. Example with satwcomic.com/latest:

1. RA fetchs [FeedBurner link](http://feeds.feedburner.com/satwcomic) (discoverable on-site)
1. G fetches `https://satwcomic.com/food-crimes`, grabs `img[itemprop]` as main `content`, and `[itemprop="articleBody"]` as extra text
1. ST is not used, the title text is the same as comic title
1. FT doesn't do any relevant transformations (feed `img` has size, but G's output does not)

The transformations I want are:

- [x] Show title text: for each `img[title]`, copy the title out as sibling `p`
- [x] Remove resizing: for each `img`, remove srcset,height,width attr, remove ComicPress resizing parameters in src attr
- Oglaf: fetch comics, show both `alt` and `title`
- Channelate: the bonus panel is on a separate page, with strange filename. Thus we run grabber for it & the bonus page

## Persistence

Inoreader remembers items per feed, i hope. It'd be nice to short `items` by some date, before letting inoreader fetch them.

Huginn stores agent settings & memory in PG. Some useful data we can get out is:

- (UI) agents' last output event age (query param `sort=last_event_at.asc`), to detect idle sources
- (DB) fetch error in log, to detect changed sites, expired certs, etc.

## Laker

rsspls and news-flash crates are possible idea sources.

we're using `miniserve` to keep the VM flying, but data is generated
before build time with `cargo run --bin oneshot`, using one-time
fetched XML files.
