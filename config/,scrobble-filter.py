#!/bin/env python3
# -*- apheleia-formatter: dprint -*-
import sys
import re

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
    ".wma",
]
if (artist == "" and album == "" and len(title) in (0, 8)) or any(
    title.lower().endswith(e) for e in EXTENSIONS
):
    # if "-" in title:
    #     artist, title = title.rsplit("-", maxsplit=1)
    # else:
    submit = False
    sys.exit(0)

COMPILATIONS: list[str] = [
    ".",
    "buddy.vn",
    "mp3.zing.vn",
    "Music",
    "www.hienld.tk/www.Nhac.vui.vn",
    "www.hienld.tk/Www.Nhac.Vui.Vn Download Music Mp3 Free Nhanh Nhat Viet Nam",
]
COMPILERS: list[str] = [
    "",
    "CASSETTE HOÀI NIỆM",
    "Danh ca hải ngoại",
    "EnPi - Yêu Nhạc Truyền Thống",
    "Francisco Callahan",
    "Hà Nội Vi Vu",
    "VTV3",
    "VTV Go",
    "VTV SHOWS",
    "VTV - Bài Ca Đi Cùng Năm Tháng",
    "khanhnguyen03",
    "khánh đi sưu tầm",
]
ARTIST_REPLACE_RULES: dict[str, str] = {
    "": ["OFFICIAL", "Official", "NSƯT", "Ca Sĩ", "NSND"],
    "Anh Thơ": ["Anh Tho"],
    "Buckethead": ["Bucketheadland"],
    "Frédéric Chopin": ["Fryderyk Chopin", "Garrick Ohlsson"],
    "Khánh Ly": ["KHÁNH LY"],
    "Lệ Quyên": ["Le Quyen"],
    "Megadriver": ["megadriver"],
    "Microwave": ["Microwave band", "MICROWAVE"],
    "Minh Thu": ["MINH THU"],
    "Mỹ Tâm": ["My Tam"],
    "Nightwish": ["NIGHTWISH"],
    "Yên Hà": ["YÊN HÀ I- CÔ GIÁO HÁT NHẠC NGA"],
}
ARTISTS = ARTIST_REPLACE_RULES.keys()

TITLE_REPLACE_RULES: dict[str, str] = {
    "": sorted(
        [
            " - Lyrics & Engsub",
            " | Audio",
            " | Hà Nội Vi Vu",
            " | MINH THU | TRỊNH XƯA",
            " | MINH THU",
            " | Official Lyric Video by Hà Nội Vi Vu",
            " | OFFICIAL MUSIC VIDEO 4K | ",
            " | OFFICIAL MV || Nhạc Xuân Trữ Tình Acoustic",
            " | OFFICIAL MV",
            " | THUỲ DUNG",
            " | TOP bài hát hay",
            " || MANH PIANO Official",
            " || THUỲ DUNG",
            " || Tình Khúc San và Trịnh HAY NỨC NỞ",
            "(Audio)",
            "(OFFICIAL AUDIO)",
            "(Official Lyric Video)",
            "(St Trịnh Công Sơn)",
            "- KHÁNH LY | OFFICIAL",
            "- Mộc San",
            "- YÊN HÀ [OFFICIAL]",
            "BẢN THU ÂM TRƯỚC 1975 | KHÁNH LY |",
            "Chị đẹp",
            "Garrick Ohlsson / Chopin:",
            "Gil Ventura-",
            "hát theo",
            "khiến cả hôi trường",
            "khiến khán giả lặng thinh",
            "khiến khán giả vỗ tay không ngớt",
            "Mộc San - Trịnh Ca || ",
            "NHẠC TRỊNH HAY ||",
            "OFFICIAL LYRICS VIDEO",
            "Official Lyrics Video",
            "OFFICIAL MUSIC VIDEO",
            "THU ÂM TRƯỚC 1975 | ",
            "| Cassette Hoài niệm | VTV3",
            "|| Official MV 4k",
            "đứng ngồi không yên",
            "⭐",
            "🎵",
            "🎵Mạnh Piano - ",
            "🎵Mạnh Piano | ",
            "🎶",
            "💎",
            "🔥",
        ],
        # drop longest matches first
        reverse=True,
        key=len,
    ),
}

if album in COMPILATIONS:
    album = ""

for replacement, matches in TITLE_REPLACE_RULES.items():
    for s in matches:
        title = title.replace(s, replacement, count=1)

if artist in COMPILERS and " | " in title:
    title, artist = title.rsplit(" | ", maxsplit=1)
if artist in COMPILERS and " - " in title:
    title, artist = title.rsplit(" - ", maxsplit=1)
    if title in ARTISTS:
        artist, title = title, artist

for replacement, matches in ARTIST_REPLACE_RULES.items():
    for s in matches:
        artist = artist.replace(s, replacement, count=1)

if ORIG_ARTIST == "Chopin":
    # avoid "fred fred chopin" from naive loop
    artist = "Frédéric Chopin"

if ORIG_ARTIST == "Vietnam War Song Project":
    if " - " in title:
        artist, title = title.rsplit(" - ", maxsplit=1)
    elif "(" in title:
        artist: str = re.sub("""^.+[(](.+)[)]$""", "\\1", ORIG_TITLE)
        title: str = ORIG_TITLE.split("(")[0]

if ORIG_ARTIST == "khánh đi sưu tầm" and " | " in title:
    title, artist, *_comment = title.rsplit(" | ", maxsplit=2)

if ORIG_ARTIST == "ogafroman":
    title: str = re.sub("""^Afroman, "(.+)"$""", "\\1", ORIG_TITLE)
    artist = "Afroman"

if ORIG_ARTIST == "Netherlands Bach Society":
    if title.startswith("Bach - "):
        title: str = title.replace("Bach - ", "", count=1)
    title: str = title.replace(" | Netherlands Bach Society", "")
    artist = "Johann Sebastian Bach"

if ORIG_ARTIST.startswith("Pike ") and " - " in ORIG_ARTIST:
    title: str = ORIG_ARTIST.split(" - ")[1]
    artist = "Buckethead"

if artist == "" and "Micro Lesson" in title:
    artist = "Scott Adams"

artist: str = artist.strip()
album: str = album.strip()
title: str = title.strip()

print(
    f"""
OA:      {ORIG_ARTIST}

OT:       {ORIG_TITLE}
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
