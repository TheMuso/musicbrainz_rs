use serde::{Deserialize, Serialize};

/// Genres are currently supported in MusicBrainz as part of the tag system.
/// See [Genre](https://musicbrainz.org/doc/Genre) and
/// [supported genres](https://musicbrainz.org/genres) for more information.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "kebab-case"))]
pub struct Genre {
    pub count: u32,
    pub name: String,
}
