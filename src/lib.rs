#![doc = include_str!("../README.md")]

use serde::Deserialize;

/// Chapters follow mostly the [Podcast namespace specification](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
#[derive(Debug, PartialEq)]
pub struct Chapter {
    /// The starting time of the chapter.
    pub start: chrono::Duration,
    /// The end time of the chapter.
    pub end: Option<chrono::Duration>,
    /// The title of this chapter.
    pub title: Option<String>,
    /// The url of an image to use as chapter art.
    pub image_url: Option<url::Url>,
    /// The url of a web page or supporting document that's related to the topic of this chapter.
    pub url: Option<url::Url>,
    /// If this property is set to true, this chapter should not display visibly to the user in either the table of contents or as a jump-to point in the user interface. In the original spec, the inverse of this is called `toc`.
    pub hidden: bool,
    // TODO: This object defines an optional location that is tied to this chapter.
    // pub location: Option<()>,
}

impl Default for Chapter {
    fn default() -> Self {
        Self {
            start: chrono::Duration::zero(),
            end: None,
            title: None,
            image_url: None,
            url: None,
            hidden: false,
        }
    }
}

/// Chapters of the [Podcast namespace](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
#[derive(Debug, PartialEq, Deserialize)]
struct PodcastNamespaceChapters {
    pub version: String,
    pub chapters: Vec<PodcastNamespaceChapter>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PodcastNamespaceChapter {
    /// The starting time of the chapter.
    #[serde(deserialize_with = "float_to_duration")]
    pub start_time: chrono::Duration,
    /// The end time of the chapter.
    #[serde(default, deserialize_with = "float_to_duration_option")]
    pub end_time: Option<chrono::Duration>,
    /// The title of this chapter.
    #[serde(default)]
    pub title: Option<String>,
    /// The url of an image to use as chapter art.
    #[serde(default, deserialize_with = "string_to_url")]
    pub img: Option<url::Url>,
    /// The url of a web page or supporting document that's related to the topic of this chapter.
    #[serde(default, deserialize_with = "string_to_url")]
    pub url: Option<url::Url>,
    /// If this property is present and set to false, this chapter should not display visibly to the user in either the table of contents or as a jump-to point in the user interface.
    #[serde(default)]
    pub toc: Option<bool>,
    // TODO: This object defines an optional location that is tied to this chapter.
    // pub location: Option<()>,
}

fn float_to_duration_option<'de, D>(deserializer: D) -> Result<Option<chrono::Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f = match Option::<f64>::deserialize(deserializer) {
        Ok(f) => f,
        Err(e) => return Ok(None),
    };
    Ok(f.map(|f| chrono::Duration::seconds(f as i64)))
}

fn float_to_duration<'de, D>(deserializer: D) -> Result<chrono::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f = f64::deserialize(deserializer)?;
    Ok(chrono::Duration::seconds(f as i64))
}

fn string_to_url<'de, D>(deserializer: D) -> Result<Option<url::Url>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(url::Url::parse(&s).ok())
}

fn podcast_namespace_chapters_to_chapters(
    podcast_namespace_chapters: PodcastNamespaceChapters,
) -> Vec<Chapter> {
    podcast_namespace_chapters
        .chapters
        .into_iter()
        .map(|podcast_namespace_chapter| Chapter {
            start: podcast_namespace_chapter.start_time,
            end: podcast_namespace_chapter.end_time,
            title: podcast_namespace_chapter.title,
            image_url: podcast_namespace_chapter.img,
            url: podcast_namespace_chapter.url,
            hidden: !podcast_namespace_chapter.toc.unwrap_or(true),
        })
        .collect()
}

pub fn parse_chapters<R: std::io::Read>(reader: R) -> Result<Vec<Chapter>, String> {
    let podcast_namespace_chapters: PodcastNamespaceChapters =
        serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(podcast_namespace_chapters_to_chapters(
        podcast_namespace_chapters,
    ))
}

pub fn chapters_from_description(description: &str) -> Result<Vec<Chapter>, String> {
    Ok(vec![])
}
