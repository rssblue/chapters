#![doc = include_str!("../README.md")]

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

pub fn parse_chapters<R: std::io::Read>(reader: R) -> Result<Vec<Chapter>, String> {
    Ok(vec![])
}
