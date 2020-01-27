use crate::hls_structs::*;
use regex::Regex;

/**
 * TODO in this file:
 * - Check the HLS spec for treat the optional title on tags
 * - Big jobs to do here -> see goo.uri
 */

pub struct ManifestTextParser {}

/**
 * HLS manifest text parser
 */
impl ManifestTextParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_playlist(&self, data: String, absolute_playlist_uri: String) -> Playlist {
        // Normalize newlines to \n.
        let data_normalized = data.replace("/\r\n|\r(?=[^\n]|$)/gm", "\n");
        let lines: Vec<&str> = data_normalized.trim().split("\n").collect();

        let reg_header = Regex::new("^#EXTM3U($|[ \t\n])").unwrap();
        if !reg_header.is_match(lines.first().unwrap()) {
            println!("HLS header manifest is empty");
        }

        let mut playlist_type = PlaylistType::Master;

        // First, look for media playlist tags, so that we know what the playlist
        // type really is before we start parsing.
        // Whether to skip the next element; initialize to true to skip first elem.
        let mut skip = true;
        for line in &lines {
            // Ignore comments.
            if Self::is_comment(line) || skip {
                skip = false;
                continue;
            }

            let tag = Self::parse_tag(line);

            if MEDIA_PLAYLIST_TAGS.iter().any(|t| t == &tag.name) {
                playlist_type = PlaylistType::Media;
                break;
            } else if tag.name == "EXT-X-STREAM-INF" {
                skip = true;
            }
        }

        let mut tags: Vec<Tag> = Vec::new();
        // Initialize to "true" to skip the first element.
        skip = true;
        for (i, line) in lines.iter().enumerate() {
            // Ignore comments.
            if Self::is_comment(line) || skip {
                skip = false;
                continue;
            }

            let mut tag = Self::parse_tag(line);

            if SEGMENT_TAGS.iter().any(|t| t == &tag.name) {
                // Only media playlists should contain segment tags
                match playlist_type {
                    PlaylistType::Media => {
                        let segments_data: Vec<&str> = lines.clone()[i..].into();
                        let segments = Self::parse_segments(
                            absolute_playlist_uri.clone(),
                            segments_data,
                            tags.clone(),
                        );
                        Playlist::new(
                            absolute_playlist_uri.clone(),
                            playlist_type,
                            tags.clone(),
                            Some(segments),
                        );
                    }
                    _ => panic!("Only media playlists should contain segment tags"),
                }
            }

            // An EXT-X-STREAM-INF tag is followed by a URI of a media playlist.
            // Add the URI to the tag object.
            if tag.name == "EXT-X-STREAM-INF" {
                let tag_uri =
                    Attribute::new("URI".to_owned().to_string(), lines.clone()[i + 1].into());
                tag.add_attribute(tag_uri);
                skip = true;
            }

            tags.push(tag);
        }

        Playlist::new(absolute_playlist_uri, playlist_type, tags, None)
    }

    /**
     * Parses an array of strings into an array of HLS Segment objects.
     */
    fn parse_segments(
        absolute_playlist_uri: String,
        lines: Vec<&str>,
        mut playlist_tags: Vec<Tag>,
    ) -> Vec<Segment> {
        let mut segments: Vec<Segment> = Vec::new();
        let mut segments_tags: Vec<Tag> = Vec::new();

        let reg_ext = Regex::new("^#EXT").unwrap();
        for line in lines {
            if reg_ext.is_match(line) {
                let tag = Self::parse_tag(line);
                if MEDIA_PLAYLIST_TAGS.iter().any(|t| t == &tag.name) {
                    playlist_tags.push(tag);
                } else {
                    segments_tags.push(tag);
                }
            } else if Self::is_comment(line) {
                // Skip comments.
                continue;
            } else {
                let verbatim_segment_uri = line.trim();
                let absolute_segment_uri = Self::construct_absolute_uri(
                    absolute_playlist_uri.clone(),
                    verbatim_segment_uri.to_owned(),
                );
                let segment = Segment::new(absolute_segment_uri, segments_tags.clone());
                segments.push(segment);
            }
        }
        segments
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

        if line.starts_with("#") {
            // Remove #
            let (_, right) = line.split_at(1);
            // parts = [tag_name, attribute_str]
            let parts: Vec<&str> = right.split(":").collect();
            if parts.len() == 2 {
                let subparts: Vec<&str> = parts.get(1).unwrap().split(",").collect();
                let mut value = None;
                let mut attributes = None;
                // TODO: Check the HLS spec for treat the optional title on tags
                if (subparts.len() == 1 && !subparts.get(0).unwrap().contains("="))
                    || subparts.get(1).unwrap().contains("")
                {
                    value = Some(subparts.get(0).unwrap().to_owned().to_string());
                } else {
                    attributes = Some(
                        subparts
                            .into_iter()
                            .map(|subpart| {
                                let subsubpart: Vec<&str> = subpart.split("=").collect();
                                Attribute::new(
                                    subsubpart.get(0).unwrap().to_owned().to_string(),
                                    subsubpart.get(1).unwrap().to_owned().to_string(),
                                )
                            })
                            .rev()
                            .collect(),
                    );
                }
                return Tag::new(
                    parts.get(0).unwrap().to_owned().to_string(),
                    value,
                    attributes,
                );
            }
        }
        Tag::new(line.to_owned(), None, None)
    }

    /**
     * Matches a string to an HLS comment format and returns the result.
     */
    fn is_comment(line: &str) -> bool {
        let reg_comment = Regex::new("^#").unwrap();
        let reg_ext = Regex::new("^#EXT").unwrap();
        reg_comment.is_match(line) && !reg_ext.is_match(line)
    }

    fn construct_absolute_uri(parent_absolute_uri: String, uri: String) -> String {
        let mut parent_absolute_uri_vec = Vec::new();
        parent_absolute_uri_vec.push(parent_absolute_uri);
        let mut uri_vec = Vec::new();
        uri_vec.push(uri);

        Self::resolve_uris(parent_absolute_uri_vec, uri_vec).remove(0)
    }

    /**
     * Resolves an array of relative URIs to the given base URIs. This will result
     * in M*N number of URIs.
     * TODO: Big jobs to do here -> see goo.uri
     */
    fn resolve_uris(base_uris: Vec<String>, relative_uris: Vec<String>) -> Vec<String> {
        // if relative_uris.is_empty() {
        //     base_uris
        // }

        // const relativeAsGoog = relativeUris.map((uri) => new goog.Uri(uri));
        // // Resolve each URI relative to each base URI, creating an Array of Arrays.
        // // Then flatten the Arrays into a single Array.
        // return baseUris.map((uri) => new goog.Uri(uri))
        //     .map((base) => relativeAsGoog.map((i) => base.resolve(i)))
        //     .reduce(Functional.collapseArrays, [])
        //     .map((uri) => uri.toString());
        let mut vector = Vec::new();
        vector.push("yolo".to_owned());
        vector.push("yoo".to_owned());
        vector
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
