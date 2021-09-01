use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum LinkType<'a> {
    SpotifyAlbum(&'a str),
    SpotifyTrack(&'a str),
    SpotifyPlaylist(&'a str),
}

/// Let's us parse strings for spotify links, and identify what we are looking at.
/// can be used as guard for linkTypes
/// ```
/// let linkType = parse_for_links("spotify:album:72fr4Cxmz8wmWz7OAA8xZv");
/// println!("LinkType is {:?}", linkType);
/// ```
fn parse_for_links(s: &str) -> Vec<LinkType> {
    let regex: Regex = Regex::new(r"(?:(?:spotify\.com/)|(?:spotify:))(?P<type>playlist|track|album)(?:/|:)(?P<uri>[\w\d]*)")
        .unwrap();

    regex
        .captures_iter(s)
        .map(|f| -> Option<LinkType> {
            if let (Some(target_type), Some(uri)) = (f.name("type"), f.name("uri")) {
                return match target_type.as_str() {
                    "album" => Some(LinkType::SpotifyAlbum(uri.as_str())),
                    "track" => Some(LinkType::SpotifyTrack(uri.as_str())),
                    "playlist" => Some(LinkType::SpotifyPlaylist(uri.as_str())),
                    _ => None,
                };
            }
            None
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<LinkType>>()
}

#[test]
fn validate_link_parser() {
    use super::link_parser::LinkType;

        assert_eq!(parse_for_links("https://open.spotify.com/track/6xuaTUusUQ3q35YKhmpw7X?si=kc4hYPzsQiiEMf6mvR5goQ&dl_branch=1").first().unwrap(), &LinkType::SpotifyTrack("6xuaTUusUQ3q35YKhmpw7X"));
    assert_eq!(
        parse_for_links("spotify:album:72fr4Cxmz8wmWz7OAA8xZv")
            .first()
            .unwrap(),
        &LinkType::SpotifyAlbum("72fr4Cxmz8wmWz7OAA8xZv")
    );
    assert_eq!(parse_for_links("<https://open.spotify.com/track/7hfaOMuU1MOXwozgNm3wvI?si=1e19b0e645224ac2%7Csometext> more text").first()
                   .unwrap(), &LinkType::SpotifyTrack("7hfaOMuU1MOXwozgNm3wvI"));
    assert_eq!(parse_for_links("how\\nabout\\n\\n\\nmulti\\n<https://open.spotify.com/track/14SXDaFFXKhigF0qcaHl38?si=53917b1760cc4adb>\\n\\n\\nline?\\n\\n:boom:")
                   .first().unwrap(), &LinkType::SpotifyTrack("14SXDaFFXKhigF0qcaHl38"))
}

pub struct LinkParser<'a> {
    pub matches: Vec<LinkType<'a>>,
}

impl<'a> LinkParser<'a> {
    pub fn new(text: &'a str) -> LinkParser<'a> {
        LinkParser {
            matches: parse_for_links(text),
        }
    }
}
