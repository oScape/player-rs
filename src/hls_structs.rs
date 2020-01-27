/**
 * HLS playlist struct.
 */
pub struct Playlist {
    pub absolute_uri: String,
    pub playlist_type: PlaylistType,
    pub tags: Vec<Tag>,
    pub segments: Option<Vec<Segment>>,
}

impl Playlist {
    pub fn new(
        absolute_uri: String,
        playlist_type: PlaylistType,
        tags: Vec<Tag>,
        segments: Option<Vec<Segment>>,
    ) -> Self {
        Self {
            absolute_uri: absolute_uri,
            playlist_type: playlist_type,
            tags: tags,
            segments: segments,
        }
    }
}

#[derive(Copy, Clone)]
pub enum PlaylistType {
    Master = 0,
    Media = 1,
}

/**
 * HLS tag struct.
 */
#[derive(Clone)]
pub struct Tag {
    pub name: String,
    pub value: String,
    pub attributes: Vec<Attribute>,
}

impl Tag {
    pub fn new(name: String, value: Option<String>, attributes: Option<Vec<Attribute>>) -> Self {
        Self {
            name: name,
            value: value.unwrap_or_default(),
            attributes: attributes.unwrap_or_default(),
        }
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }
}

/**
 * HLS segment struct.
 */
pub struct Segment {
    pub absolute_uri: String,
    pub tags: Vec<Tag>,
}

impl Segment {
    pub fn new(absolute_uri: String, tags: Vec<Tag>) -> Self {
        Self {
            absolute_uri: absolute_uri,
            tags: tags,
        }
    }
}

/**
 * HLS Attribute struct.
 */
#[derive(Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name: name,
            value: value,
        }
    }
}
