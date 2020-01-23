mod manifest_text_parser;

fn main() {
    let manifest = std::fs::read_to_string("ext/Manifest.m3u8").expect("File not found!");
    manifest_text_parser::ManifestTextParser::new().parse_playlist(manifest, "yolo".to_owned());
}
