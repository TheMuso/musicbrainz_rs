use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::release::Release;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use serde::{Deserialize, Serialize};

use lucene_query_builder::QueryBuilder;

/// Labels are one of the most complicated and controversial parts of the music industry. The main
/// reason for that being that the term itself is not clearly defined and refers to at least two
/// overlapping concepts: imprints, and the companies that control them. Fortunately, in many cases
/// the imprint and the company controlling it have the same name.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "kebab-case"))]
pub struct Label {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub type_id: Option<String>,
    /// The type describes the main activity of the label.
    #[serde(rename = "type")]
    pub label_type: Option<LabelType>,
    /// The official name of the label.
    pub name: String,
    pub sort_name: Option<String>,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,
    pub relations: Option<Vec<Relation>>,
    pub country: Option<String>,
    /// The label code is the "LC" code of the label.
    pub label_code: Option<u32>,
    pub releases: Option<Vec<Release>>,
    /// Aliases are used to store alternate names or misspellings.
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct LabelSearchQuery {
    /// (part of) any alias attached to the label (diacritics are ignored)
    pub alias: String,
    /// (part of) the name of the label's main associated area
    pub area: String,
    /// the label's begin date (e.g. "1980-01-22")
    pub begin: String,
    /// the label code for the label (only the numbers, without "LC")
    pub code: String,
    /// (part of) the label's disambiguation comment
    pub comment: String,
    /// the 2-letter code (ISO 3166-1 alpha-2) for the label's associated country
    pub country: String,
    /// the label's end date (e.g. "1980-01-22")
    pub end: String,
    /// a boolean flag (true/false) indicating whether or not the label has ended (is dissolved)
    pub ended: String,
    /// an IPI code associated with the label
    pub ipi: String,
    /// an ISNI code associated with the label
    pub isni: String,
    /// (part of) the label's name (diacritics are ignored)
    pub label: String,
    /// (part of) the label's name (with the specified diacritics)
    #[query_builder_field = "labelaccent"]
    pub label_accent: String,
    /// the label's MBID
    pub laid: String,
    /// the amount of releases related to the label
    pub release_count: String,
    /// equivalent to name (labels no longer have separate sort names)
    #[query_builder_field = "sortname"]
    pub sort_name: String,
    /// (part of) a tag attached to the label
    pub tag: String,
    /// the label's type
    #[query_builder_field = "type"]
    pub label_type: Option<LabelType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "kebab-case", serialize = "kebab-case"))]
pub struct LabelInfo {
    pub catalog_number: Option<String>,
    pub label: Option<Label>,
}

/// The type of a MusicBrainz label entity.
/// Note that this enum is `non_exhaustive`; The list of label types is subject to change and these
/// changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `label_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LabelType {
    #[serde(rename = "Bootleg Production")]
    BootlegProduction,
    Distributor,
    Holding,
    Imprint,
    #[serde(rename = "Original Production")]
    OriginalProduction,
    Production,
    Publisher,
    #[serde(rename = "Reissue Production")]
    ReissueProduction,
    #[serde(rename = "Rights Society")]
    RightsSociety,
    Manufacturer,
    /// Any label_type that does not yet have a corresponding variant in this enum.
    /// If you ever see a `LabelType::UnrecognizedLabelType` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedLabelType,
}

impl_browse! {
Label,
   (by_area, BrowseBy::Area),
   (by_release, BrowseBy::Release),
   (by_collection, BrowseBy::Collection)
}

impl_includes!(
    Label,
    (
        with_artist_relations,
        Include::Relationship(Relationship::Artist)
    ),
    (
        with_label_relations,
        Include::Relationship(Relationship::Label)
    ),
    (
        with_recording_relations,
        Include::Relationship(Relationship::Recording)
    ),
    (
        with_release_relations,
        Include::Relationship(Relationship::Release)
    ),
    (with_url_relations, Include::Relationship(Relationship::Url)),
    (with_releases, Include::Subquery(Subquery::Releases)),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);
