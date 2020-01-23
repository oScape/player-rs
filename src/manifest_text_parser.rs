use regex::Regex;

pub struct ManifestTextParser {}

/**
 * HLS manifest text parser
 */
impl ManifestTextParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_playlist(&self, data: String) {
        // Normalize newlines to \n.
        let lines = data.lines();
        let reg_header = Regex::new("^#EXTM3U($|[ \t\n])").unwrap();

        if !reg_header.is_match(lines.clone().nth(0).unwrap()) {
            println!("HLS header manifest is empty");
        }

        let playlistType = PlaylistType::Master;
        let mut skip = true;

        for line in lines {
            // Ignore comments.
            if Self::is_comment(line) || skip {
                skip = false;
                continue;
            }
            // let tag = Self::parse_tag(line);
        }
    }

    /**
     * Matches a string to an HLS comment format and returns the result.
     */
    fn is_comment(line: &str) -> bool {
        if line.starts_with("#EXT") {
            false
        } else if line.starts_with("#") {
            true
        } else {
            false
        }
    }

    /**
     * Parses a string into an HLS Tag struct.
     */
    fn parse_tag(line: &str) -> Tag {
        // HLS tags start with '#EXT'. A tag can have a set of attributes
        // (#EXT-<tagname>:<attribute list>) and/or a value (#EXT-<tagname>:<value>).
        // An attribute's format is 'AttributeName=AttributeValue'.
        // The parsing logic goes like this:
        // 1. Everything before ':' is a name (we ignore '#').
        // 2. Everything after ':' is a list of comma-seprated items,
        //   2a. The first item might be a value, if it does not contain '='.
        //   2b. Otherwise, items are attributes.
        // 3. If there is no ":", it's a simple tag with no attributes and no value.
        let tag: &str = line[line.find("#").unwrap() + 1..].into();
        let name: &str = tag[..tag.find(":").unwrap()].into();
        let data: &str = tag[tag.find(":").unwrap() + 1..].into();

        if data.len() > 0 {}
        Tag::new()
    }
}

struct Tag {}

impl Tag {
    pub fn new() -> Self {
        Self {}
    }
}

struct TextParser {}

impl TextParser {
    pub fn new() -> Self {
        Self {}
    }
}

/**
 * HLS tags that only appear on Media Playlists.
 * Used to determine a playlist type.
 */
static MEDIA_PLAYLIST_TAGS: &[&str] = &[
    "EXT-X-TARGETDURATION",
    "EXT-X-MEDIA-SEQUENCE",
    "EXT-X-DISCONTINUITY-SEQUENCE",
    "EXT-X-PLAYLIST-TYPE",
    "EXT-X-MAP",
    "EXT-X-I-FRAMES-ONLY",
    "EXT-X-ENDLIST",
];

/**
 * HLS tags that only appear on Segments in a Media Playlists.
 * Used to determine the start of the segments info.
 */
static SEGMENT_TAGS: &[&str] = &[
    "EXTINF",
    "EXT-X-BYTERANGE",
    "EXT-X-DISCONTINUITY",
    "EXT-X-PROGRAM-DATE-TIME",
    "EXT-X-KEY",
    "EXT-X-DATERANGE",
];

enum PlaylistType {
    Master = 0,
    Media = 1,
}
