#!/bin/env python3
# -*- apheleia-formatter: dprint -*-
import sys

# protocol: https://github.com/inputusername/rescrobbled?tab=readme-ov-file#configuration
submit: bool = True

ORIG_ARTIST, ORIG_TITLE, ORIG_ALBUM, *_rest = (
    l.rstrip() for l in sys.stdin.readlines()
)
artist: str = ORIG_ARTIST
title: str = ORIG_TITLE
album: str = ORIG_ALBUM

EXTENSIONS: list[str] = [
    ".alac",
    ".flac",
    ".m3u8",
    ".m4a",
    ".mp3",
    ".mp4",
    ".ogg",
    ".opus",
    ".pcx",
    ".smk",
    ".vgz",
    ".wav",
]
if (artist == "" and album == "" and len(title) in (0, 8)) or any(
    title.lower().endswith(e) for e in EXTENSIONS
):
    # if "-" in title:
    #     artist, title = title.rsplit("-", maxsplit=1)
    # else:
    submit = False
    sys.exit(0)

COMPILATIONS: list[str] = ["buddy.vn", ".", "Music", "mp3.zing.vn"]
COMPILERS: list[str] = [
    "Danh ca hải ngoại",
    "Francisco Callahan",
    "Hà Nội Vi Vu",
    "VTV - Bài Ca Đi Cùng Năm Tháng",
    "VTV Go",
    "VTV SHOWS",
    "VTV3",
    "khanhnguyen03",
]
FLIP_COMPILERS: list[str] = [
    "",
    "ogafroman",
    "Vietnam War Song Project",
]
ARTIST_REPLACE_RULES: dict[str, str] = {
    "": ["OFFICIAL", "Official"],
    "Afroman": ["ogafroman"],
    "Anh Thơ": ["Anh Tho"],
    "Buckethead": ["Bucketheadland"],
    "Bích Liên": ["NSƯT Bích Liên"],
    "Frédéric Chopin": ["Fryderyk Chopin"],
    "Johann Sebastian Bach": ["Netherlands Bach Society"],
    "Khánh Ly": ["Ca Sĩ KHÁNH LY", "Ca Sĩ KHÁNH LY"],
    "Megadriver": ["megadriver"],
    "Microwave": ["Microwave band", "MICROWAVE"],
    "Minh Thu": ["MINH THU"],
    "Mỹ Tâm": ["My Tam"],
    "Yên Hà": ["YÊN HÀ I- CÔ GIÁO HÁT NHẠC NGA"],
}
ARTISTS = ARTIST_REPLACE_RULES.keys()

TITLE_REPLACE_RULES: dict[str, str] = {
    "": sorted(
        [
            "BẢN THU ÂM TRƯỚC 1975 | KHÁNH LY |",
            "Chị đẹp",
            "Garrick Ohlsson / Chopin:",
            "Mộc San - Trịnh Ca || ",
            "NHẠC TRỊNH HAY ||",
            "OFFICIAL LYRICS VIDEO",
            "OFFICIAL MUSIC VIDEO",
            "Official Lyrics Video",
            "THU ÂM TRƯỚC 1975 | ",
            "hát theo",
            "khiến cả hôi trường",
            " - Lyrics & Engsub",
            " | Audio",
            " | MINH THU | TRỊNH XƯA",
            " | MINH THU",
            " | Netherlands Bach Society",
            " | OFFICIAL MUSIC VIDEO 4K | ",
            " | Official Lyric Video by Hà Nội Vi Vu",
            " || MANH PIANO Official",
            " || Tình Khúc San và Trịnh HAY NỨC NỞ",
            "(Audio)",
            "(OFFICIAL AUDIO)",
            "(Official Lyric Video)",
            "(St Trịnh Công Sơn)",
            "- KHÁNH LY | OFFICIAL",
            "- Mộc San",
            "- YÊN HÀ [OFFICIAL]",
            "|| Official MV 4k",
            "đứng ngồi không yên",
            "⭐",
            "🎵Mạnh Piano | ",
            "💎",
        ],
        reverse=True,
        key=len,
    ),
}

if album in COMPILATIONS:
    album = ""

for replacement, matches in ARTIST_REPLACE_RULES.items():
    for s in matches:
        artist = artist.replace(s, replacement, count=1)

for replacement, matches in TITLE_REPLACE_RULES.items():
    for s in matches:
        title = title.replace(s, replacement, count=1)

if artist in FLIP_COMPILERS and " - " in title:
    artist, title = title.rsplit(" - ", maxsplit=1)
if artist in COMPILERS and " | " in title:
    title, artist = title.rsplit(" | ", maxsplit=1)
if artist in COMPILERS and " - " in title:
    title, artist = title.rsplit(" - ", maxsplit=1)
    if title in ARTISTS:
        artist, title = title, artist
if artist.startswith("Pike "):
    title = f"{artist} {title}"
    artist = "Buckethead"
if artist == "" and "Micro Lesson" in title:
    artist = "Scott Adams"

artist = artist.strip()
album = album.strip()
title = title.strip()

print(
    f"""
      {ORIG_ARTIST}

       {ORIG_TITLE}
     title         : {title}
  artist           : {artist}
     album         : {album}
""",
    end="\n\n",
    flush=True,
    file=sys.stderr,
)

if artist == "":
    submit = False

if submit:
    print(artist, title, album, sep="\n")
