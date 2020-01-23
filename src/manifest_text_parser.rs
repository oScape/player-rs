use regex::Regex;
use std::cmp::PartialEq;

pub struct ManifestTextParser {}

/**
 * HLS manifest text parser
 */
impl ManifestTextParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_playlist(&self, data: String, absolutePlaylistUri: String) {
        // Normalize newlines to \n.
        let data_normalized = data.replace("/\r\n|\r(?=[^\n]|$)/gm", "\n");
        let data_splitted = data_normalized.trim().split("/\n+/m");
        let lines: Vec<&str> = data_splitted.collect();

        let reg_header = Regex::new("^#EXTM3U($|[ \t\n])").unwrap();
        if !reg_header.is_match(lines.first().unwrap()) {
            println!("HLS header manifest is empty");
        }

        let mut playlistType = PlaylistType::Master;

        // First, look for media playlist tags, so that we know what the playlist type really is before we start parsing.
        // Whether to skip the next element; initialize to true to skip first elem.
        let mut skip = true;
        for line in lines.clone() {
            println!("{:#}", line);
            // Ignore comments.
            if Self::is_comment(line) || skip {
                skip = false;
                continue;
            }

            let tag = Self::parse_tag(line);

            if MEDIA_PLAYLIST_TAGS.iter().any(|t| t == &tag.name) {
                playlistType = PlaylistType::Media;
                break;
            } else if tag.name == "EXT-X-STREAM-INF" {
                skip = true;
            }
        }

        let mut tags: Vec<Tag> = Vec::new();
        skip = true;
        for (i, line) in lines.iter().enumerate() {
            // Ignore comments.
            if Self::is_comment(line) || skip {
                skip = false;
                continue;
            }

            let tag = Self::parse_tag(line);
            if MEDIA_PLAYLIST_TAGS.iter().any(|t| t == &tag.name) {
                // match playlistType {
                //     PlaylistType::Media => {
                //         let segmentsData: Vec<&str> = lines[..i].into();
                //         let segments =
                //             Self::parse_segments(&absolutePlaylistUri, segmentsData, tags);
                //         Playlist::new(absolutePlaylistUri.clone(), playlistType, tags, segments);
                //     }
                //     _ => println!("Only media playlists should contain segment tags"),
                // }
            }
        }
    }

    /**
     * Matches a string to an HLS comment format and returns the result.
     */
    fn is_comment(line: &str) -> bool {
        let reg_comment = Regex::new("^#").unwrap();
        let reg_ext = Regex::new("^#EXT").unwrap();
        reg_comment.is_match(line) && !reg_ext.is_match(line)
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
        Tag::new(name.to_owned())
    }

    /**
     * Parses an array of strings into an array of HLS Segment objects.
     */
    fn parse_segments(
        absoluteMediaPlaylistUri: &String,
        lines: Vec<&str>,
        playlistTags: Vec<Tag>,
    ) -> Vec<Segment> {
        let mut segments: Vec<Segment> = Vec::new();
        segments.push(Segment::new());
        segments
    }
}

/**
 * HLS tag struct.
 */
struct Tag {
    name: String,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self { name: name }
    }
}

/**
 * HLS segment struct.
 */
struct Segment {}

impl Segment {
    pub fn new() -> Self {
        Self {}
    }
}

/**
 * HLS playlist struct.
 */
struct Playlist {
    absoluteUri: String,
    playlistType: PlaylistType,
    tags: Vec<Tag>,
    segments: Vec<Segment>,
}

impl Playlist {
    pub fn new(
        absoluteUri: String,
        playlistType: PlaylistType,
        tags: Vec<Tag>,
        segments: Vec<Segment>,
    ) -> Self {
        Self {
            absoluteUri: absoluteUri,
            playlistType: playlistType,
            tags: tags,
            segments: segments,
        }
    }
}

/**
 * Reads elements from strings.
 */
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
const MEDIA_PLAYLIST_TAGS: [&'static str; 7] = [
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
const SEGMENT_TAGS: [&'static str; 6] = [
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
