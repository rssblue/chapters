#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod serialization;

use chrono::Duration;
use id3::{Error, ErrorKind, Tag, TagLike, Version};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// Represents a web link for the [chapter](crate::Chapter).
#[derive(Debug, PartialEq, Serialize)]
pub struct Link {
    /// The URL of the link.
    #[serde(serialize_with = "serialization::url_to_string")]
    pub url: url::Url,
    /// The title of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Represents a [chapter](crate::Chapter) image.
#[derive(Debug, PartialEq)]
pub enum Image {
    /// The URL of the image.
    Url(url::Url),
    // TODO: some ways of encoding chapters (e.g., ID3 tags in MP3 files) allow to embed images directly in the file.
    // Data(Vec<u8>),
}

/// Represents a remote item as defined in the [Podcast namespace
/// specification](https://podcastindex.org/namespace/1.0#remote-item). Used internally by RSS
/// Blue.
#[derive(Debug, PartialEq, Serialize)]
pub enum RemoteEntity {
    /// Represents a podcast feed.
    #[serde(rename = "feed")]
    Feed {
        /// [Podcast GUID](https://podcastindex.org/namespace/1.0#guid)
        guid: Uuid,
    },
    /// Represents a podcast item.
    #[serde(rename = "item")]
    Item {
        /// [Podcast GUID](https://podcastindex.org/namespace/1.0#guid)
        feed_guid: Uuid,
        /// Item GUID, see <https://www.rssboard.org/rss-specification>.
        guid: String,
    },
}

/// Chapters follow mostly the [Podcast namespace specification](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
#[derive(Debug, PartialEq, Serialize)]
pub struct Chapter {
    /// The starting time of the chapter.
    #[serde(serialize_with = "serialization::duration_to_float")]
    pub start: Duration,
    /// The end time of the chapter.
    #[serde(
        serialize_with = "serialization::duration_option_to_float_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub end: Option<Duration>,
    /// The title of this chapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The image to use as chapter art.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// Web page or supporting document that's related to the topic of this chapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    /// If this property is set to true, this chapter should not display visibly to the user in either the table of contents or as a jump-to point in the user interface. In the original spec, the inverse of this is called `toc`.
    pub hidden: bool,
    // TODO: This object defines an optional location that is tied to this chapter.
    // pub location: Option<()>,
    /// Remote entity used internally by RSS Blue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_entity: Option<RemoteEntity>,
}

impl Default for Chapter {
    fn default() -> Self {
        Self {
            start: Duration::zero(),
            end: None,
            title: None,
            image: None,
            link: None,
            hidden: false,
            remote_entity: None,
        }
    }
}

impl From<PodcastNamespaceChapter> for Chapter {
    fn from(podcast_namespace_chapter: PodcastNamespaceChapter) -> Self {
        Self {
            start: podcast_namespace_chapter.start_time,
            end: podcast_namespace_chapter.end_time,
            title: podcast_namespace_chapter.title,
            image: podcast_namespace_chapter.img.map(Image::Url),
            link: podcast_namespace_chapter
                .url
                .map(|url| Link { url, title: None }),
            hidden: !podcast_namespace_chapter.toc.unwrap_or(true),
            remote_entity: None,
        }
    }
}

/// Chapters of the [Podcast namespace](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct PodcastNamespaceChapters {
    version: String,
    chapters: Vec<PodcastNamespaceChapter>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PodcastNamespaceChapter {
    /// The starting time of the chapter.
    #[serde(
        deserialize_with = "serialization::float_to_duration",
        serialize_with = "serialization::duration_to_float"
    )]
    start_time: Duration,
    /// The end time of the chapter.
    #[serde(
        default,
        deserialize_with = "serialization::float_to_duration_option",
        serialize_with = "serialization::duration_option_to_float_option",
        skip_serializing_if = "Option::is_none"
    )]
    end_time: Option<Duration>,
    /// The title of this chapter.
    #[serde(default)]
    title: Option<String>,
    /// The url of an image to use as chapter art.
    #[serde(
        default,
        deserialize_with = "serialization::string_to_url",
        serialize_with = "serialization::url_option_to_string",
        skip_serializing_if = "Option::is_none"
    )]
    img: Option<url::Url>,
    /// The url of a web page or supporting document that's related to the topic of this chapter.
    #[serde(
        default,
        deserialize_with = "serialization::string_to_url",
        serialize_with = "serialization::url_option_to_string",
        skip_serializing_if = "Option::is_none"
    )]
    url: Option<url::Url>,
    /// If this property is present and set to false, this chapter should not display visibly to the user in either the table of contents or as a jump-to point in the user interface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    toc: Option<bool>,
    // TODO: This object defines an optional location that is tied to this chapter.
    // pub location: Option<()>,
}

impl<'a> From<&'a Chapter> for PodcastNamespaceChapter {
    fn from(chapter: &'a Chapter) -> Self {
        Self {
            start_time: chapter.start,
            end_time: chapter.end,
            title: chapter.title.clone(),
            img: match &chapter.image {
                Some(Image::Url(url)) => Some(url.clone()),
                _ => None,
            },
            url: chapter.link.as_ref().map(|link| link.url.clone()),
            toc: if chapter.hidden { Some(false) } else { None },
        }
    }
}

/// Reads [chapters](crate::Chapter) from a [JSON chapters file](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
///
/// # Example:
/// ```rust
/// # use chapters::{Chapter, Image, Link};
/// # use chrono::Duration;
/// # use pretty_assertions::assert_eq;
/// #
/// # fn main() {
/// let json = r#"{
///   "version": "1.2.0",
///   "chapters": [
///     {
///       "startTime": 0,
///       "endTime": 30.5,
///       "title": "Chapter 1",
///       "img": "https://example.com/chapter-1.jpg",
///       "url": "https://example.com/chapter-1"
///     },
///     {
///       "startTime": 30.5,
///       "title": "Chapter 2"
///     },
///     {
///       "startTime": 55,
///       "title": "Hidden chapter",
///       "toc": false
///     },
///     {
///       "startTime": 60,
///       "endTime": 90,
///       "title": "Chapter 3",
///       "img": "https://example.com/chapter-3.jpg"
///     }
///   ]
/// }"#;
///
/// let chapters = chapters::from_json(json.as_bytes()).unwrap();
///
/// assert_eq!(
///     chapters,
///     vec![
///         Chapter {
///             start: Duration::seconds(0),
///             end: Some(Duration::seconds(30) + Duration::milliseconds(500)),
///             title: Some("Chapter 1".to_string()),
///             image: Some(Image::Url(
///                 url::Url::parse("https://example.com/chapter-1.jpg").unwrap()
///             )),
///             link: Some(Link {
///                 url: url::Url::parse("https://example.com/chapter-1").unwrap(),
///                 title: None,
///             }),
///             ..Default::default()
///         },
///         Chapter {
///             start: Duration::seconds(30) + Duration::milliseconds(500),
///             end: None,
///             title: Some("Chapter 2".to_string()),
///             ..Default::default()
///         },
///         Chapter {
///             start: Duration::seconds(55),
///             end: None,
///             title: Some("Hidden chapter".to_string()),
///             hidden: true,
///             ..Default::default()
///         },
///         Chapter {
///             start: Duration::seconds(60),
///             end: Some(Duration::seconds(90)),
///             title: Some("Chapter 3".to_string()),
///             image: Some(Image::Url(
///                 url::Url::parse("https://example.com/chapter-3.jpg").unwrap()
///             )),
///             ..Default::default()
///         },
///     ]
/// );
/// # }
/// ```
pub fn from_json<R: std::io::Read>(reader: R) -> Result<Vec<Chapter>, String> {
    let podcast_namespace_chapters: PodcastNamespaceChapters =
        serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(podcast_namespace_chapters
        .chapters
        .into_iter()
        .map(|c| c.into())
        .collect())
}

/// Writes [chapters](crate::Chapter) to a [JSON chapters file](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
///
/// # Example:
/// ```rust
/// # use chapters::{Chapter, Image, Link};
/// # use chrono::Duration;
/// # use pretty_assertions::assert_eq;
/// #
/// # fn main() {
/// let chapters = vec![
///    Chapter {
///        start: Duration::zero(),
///        title: Some("Chapter 1".to_string()),
///        ..Default::default()
///    },
///    Chapter {
///        start: Duration::seconds(45),
///        title: Some("Chapter 2".to_string()),
///        link: Some(Link {
///            url: "https://example.com".parse().unwrap(),
///            title: Some("Example".to_string()),
///        }),
///        ..Default::default()
///    },
///    Chapter {
///        start: Duration::minutes(1)+Duration::seconds(5),
///        title: Some("Hidden chapter".to_string()),
///        hidden: true,
///        ..Default::default()
///    },
///    Chapter {
///        start: Duration::minutes(2)+Duration::seconds(10)+Duration::milliseconds(500),
///        title: Some("Chapter 3".to_string()),
///        image: Some(Image::Url("https://example.com/image.png".parse().unwrap())),
///        ..Default::default()
///    },
/// ];
///
/// let json_chapters = chapters::to_json(&chapters).expect("Failed to serialize chapters");
///
/// assert_eq!(json_chapters, r#"{
///   "version": "1.2.0",
///   "chapters": [
///     {
///       "startTime": 0.0,
///       "title": "Chapter 1"
///     },
///     {
///       "startTime": 45.0,
///       "title": "Chapter 2",
///       "url": "https://example.com/"
///     },
///     {
///       "startTime": 65.0,
///       "title": "Hidden chapter",
///       "toc": false
///     },
///     {
///       "startTime": 130.5,
///       "title": "Chapter 3",
///       "img": "https://example.com/image.png"
///     }
///   ]
/// }"#);
/// # }
/// ```
pub fn to_json(chapters: &[Chapter]) -> Result<String, String> {
    let podcast_namespace_chapters = PodcastNamespaceChapters {
        version: "1.2.0".to_string(),
        chapters: chapters.iter().map(|c| c.into()).collect(),
    };
    serde_json::to_string_pretty(&podcast_namespace_chapters).map_err(|e| e.to_string())
}

/// Timestamp format used in episode descriptions.
#[derive(Debug, Clone)]
enum TimestampType {
    /// MM:SS format, e.g., "12:34"
    MmSs,
    /// HH:MM:SS format, e.g., "01:23:45"
    HhMmSs,
    /// MM:SS format within parentheses, e.g., "(12:34)"
    MmSsParentheses,
    /// HH:MM:SS format within parentheses, e.g., "(01:23:45)"
    HhMmSsParentheses,
}

impl TimestampType {
    fn regex_pattern(&self) -> &str {
        match self {
            Self::MmSs => r"^(?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)",
            Self::HhMmSs => r"^(?P<hours>\d{2}):(?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)",
            Self::MmSsParentheses => r"^\((?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)\)",
            Self::HhMmSsParentheses => {
                r"^\((?P<hours>\d{2}):(?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)\)"
            }
        }
    }

    fn line_regex_pattern(&self) -> String {
        // Combines the timestamp regex pattern with space (or a punctuation mark) and a pattern for text following the timestamp.
        format!("{}[.!?\\- ]+(?P<text>.+)$", self.regex_pattern())
    }

    fn from_line(line: &str) -> Option<Self> {
        if let Some(first_char) = line.chars().next() {
            // regex can be expensive, so we first check if the line at least starts with the right character.
            if first_char == '(' || first_char.is_numeric() {
                return [
                    Self::MmSs,
                    Self::HhMmSs,
                    Self::MmSsParentheses,
                    Self::HhMmSsParentheses,
                ]
                .iter()
                .find(|&temp_timestamp_type| {
                    regex::Regex::new(temp_timestamp_type.line_regex_pattern().as_str())
                        .map(|re| re.captures(line).is_some())
                        .unwrap_or(false)
                })
                .cloned();
            }
        }
        None
    }
}

/// Reads [chapters](crate::Chapter) from [episode description](https://help.spotifyforpodcasters.com/hc/en-us/articles/13194991130779-Enabling-podcast-chapters-) (show notes).
///
/// # Example:
/// ```rust
/// # use pretty_assertions::assert_eq;
/// #
/// # fn main() {
/// let description = r#"
/// In this episode, we explore a hot new trend in fitness: "The Movement"!
///
/// 00:00 - The Movement
/// 05:04 - Baboons
/// 09:58 - Steve Jobs
/// "#;
///
/// let chapters = chapters::from_description(description).expect("Failed to parse chapters");
///
/// assert_eq!(chapters.len(), 3);
/// assert_eq!(chapters[1].title, Some(String::from("Baboons")));
/// # }
/// ```
pub fn from_description(description: &str) -> Result<Vec<Chapter>, String> {
    let mut chapters = Vec::new();
    let mut timestamp_type: Option<TimestampType> = None;

    let parse_line = |line: &str, timestamp_type: &TimestampType| -> Option<Chapter> {
        let re = regex::Regex::new(timestamp_type.line_regex_pattern().as_str())
            .map_err(|e| e.to_string())
            .ok()?;

        if let Some(captures) = re.captures(line) {
            let start = parse_timestamp(&captures).ok()?;
            let text = captures.name("text").unwrap().as_str();
            Some(Chapter {
                start,
                end: None,
                title: Some(text.trim().to_string()),
                image: None,
                link: None,
                hidden: false,
                remote_entity: None,
            })
        } else {
            None
        }
    };

    for line in description.lines().map(|line| line.trim()) {
        if timestamp_type.is_none() {
            timestamp_type = TimestampType::from_line(line);
        }

        if let Some(timestamp_type) = timestamp_type.as_ref() {
            if let Some(chapter) = parse_line(line, timestamp_type) {
                chapters.push(chapter);
            } else {
                break;
            }
        }
    }

    Ok(chapters)
}

/// Writes [chapters](crate::Chapter) to [episode description](https://help.spotifyforpodcasters.com/hc/en-us/articles/13194991130779-Enabling-podcast-chapters-) (show notes).
///
/// Only the start time and title are used.
///
/// # Example:
/// ```rust
/// # use chapters::{Chapter, Link};
/// # use chrono::Duration;
/// # use pretty_assertions::assert_eq;
/// #
/// # fn main() {
/// let chapters = vec![
///     Chapter {
///         start: Duration::zero(),
///         title: Some("The Movement".to_string()),
///         link: Some(Link {
///             url: url::Url::parse("https://example.com/the-movement").unwrap(),
///             title: None,
///         }),
///         ..Default::default()
///     },
///     Chapter {
///         start: Duration::minutes(5) + Duration::seconds(4),
///         title: Some("Baboons".to_string()),
///         ..Default::default()
///     },
///     Chapter {
///         start: Duration::minutes(9) + Duration::seconds(58),
///         title: Some("Steve Jobs".to_string()),
///         ..Default::default()
///     },
/// ];
///
/// let description = chapters::to_description(&chapters).expect("Failed to write chapters");
/// assert_eq!(
///     description,
///     r#"00:00 The Movement
/// 05:04 Baboons
/// 09:58 Steve Jobs
/// "#
/// );
/// # }
///    ```
pub fn to_description(chapters: &[Chapter]) -> Result<String, String> {
    let mut description = String::new();

    let at_least_an_hour = chapters
        .iter()
        .any(|chapter| chapter.start >= Duration::hours(1));
    let timestamp_type = if at_least_an_hour {
        TimestampType::HhMmSs
    } else {
        TimestampType::MmSs
    };

    for chapter in chapters {
        let start = chapter.start;
        let title = chapter.title.as_ref().ok_or("Chapter title is missing")?;
        let line = format!(
            "{} {}",
            duration_to_timestamp(start, timestamp_type.clone()),
            title
        );
        description.push_str(&line);
        description.push('\n');
    }

    Ok(description)
}

fn parse_timestamp(captures: &regex::Captures) -> Result<Duration, String> {
    let parse_i64 = |capture: Option<regex::Match>| -> Result<i64, String> {
        capture
            .map(|m| m.as_str().parse::<i64>().map_err(|e| e.to_string()))
            .unwrap_or(Ok(0))
    };

    let hours = parse_i64(captures.name("hours"))?;
    let minutes = parse_i64(captures.name("minutes"))?;
    let seconds = parse_i64(captures.name("seconds"))?;

    Ok(Duration::hours(hours) + Duration::minutes(minutes) + Duration::seconds(seconds))
}

fn duration_to_timestamp(duration: Duration, timestamp_type: TimestampType) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - hours * 60;
    let seconds = duration.num_seconds() - minutes * 60 - hours * 3600;

    match timestamp_type {
        TimestampType::MmSs => format!("{minutes:02}:{seconds:02}"),
        TimestampType::HhMmSs => format!("{hours:02}:{minutes:02}:{seconds:02}"),
        TimestampType::MmSsParentheses => format!("({minutes:02}:{seconds:02})"),
        TimestampType::HhMmSsParentheses => format!("({hours:02}:{minutes:02}:{seconds:02})"),
    }
}

/// Reads [chapters](crate::Chapter) from MP3 file's [ID3](https://en.wikipedia.org/wiki/ID3) tag frames.
///
/// # Example:
/// ```rust
/// # use chapters::{Chapter, Link};
/// # use pretty_assertions::assert_eq;
/// #
/// # fn main() {
/// #     struct Test {
/// #         file_path: &'static str,
/// #         expected_chapters: Vec<Chapter>,
/// #     }
/// #
/// #     let tests = vec![
/// #         Test {
/// #         file_path: "tests/data/id3-chapters.jfk-rice-university-speech.mp3",
/// #         expected_chapters: vec![
/// #             Chapter {
/// #                 start: chrono::Duration::seconds(0),
/// #                 title: Some(String::from("Introduction")),
/// #                 ..Default::default()
/// #             },
/// #             Chapter {
/// #                 start: chrono::Duration::seconds(9),
/// #                 title: Some(String::from("Thanks")),
/// #                 ..Default::default()
/// #             },
/// #             Chapter {
/// #                 start: chrono::Duration::seconds(42),
/// #                 title: Some(String::from("Status quo")),
/// #                 ..Default::default()
/// #             },
/// #             Chapter {
/// #                 start: chrono::Duration::minutes(5) + chrono::Duration::seconds(8),
/// #                 title: Some(String::from("On being first")),
/// #                 link: Some(Link{
/// #                     url: url::Url::parse("https://www.osti.gov/opennet/manhattan-project-history/Events/1945/trinity.htm").unwrap(),
/// #                     title: Some(String::from("The Trinity Test")),
/// #                 }),
/// #                 ..Default::default()
/// #             },
/// #             Chapter {
/// #                 start: chrono::Duration::minutes(8) + chrono::Duration::seconds(8),
/// #                 title: Some(String::from("Why we're going to the Moon")),
/// #                 link: Some(Link{
/// #                     url: url::Url::parse("https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html").unwrap(),
/// #                     title: None,
/// #                 }),
/// #                 ..Default::default()
/// #             },
/// #             Chapter {
/// #                 start: chrono::Duration::minutes(16) + chrono::Duration::seconds(24),
/// #                 title: Some(String::from("Conclusion")),
/// #                 ..Default::default()
/// #             },
/// #         ],
/// #     },
/// #         Test {
/// #             file_path: "tests/data/id3-chapters.jfk-rice-university-speech.no-frames.mp3",
/// #             expected_chapters: vec![],
/// #         },
/// #     ];
/// #
/// #     for test in tests {
/// #         let path = std::path::Path::new(test.file_path);
/// let chapters = chapters::from_mp3_file(path).expect("Failed to parse chapters");
/// #
/// #        assert_eq!(chapters, test.expected_chapters);
/// #     }
/// # }
pub fn from_mp3_file<P: AsRef<Path>>(path: P) -> Result<Vec<Chapter>, String> {
    let tag = Tag::read_from_path(&path).map_err(|e| {
        format!(
            "Error reading ID3 tag from `{}`: {}",
            path.as_ref().display(),
            e
        )
    })?;
    let mut chapters = Vec::new();

    for id3_chapter in tag.chapters() {
        let start = Duration::milliseconds(id3_chapter.start_time as i64);

        let temp_end = Duration::milliseconds(id3_chapter.end_time as i64);
        // Some programs might encode chapters as instants, i.e., with the start and end time being the same.
        let end = if temp_end == start {
            None
        } else {
            Some(temp_end)
        };

        let mut title = None;
        let mut link = None;

        for subframe in &id3_chapter.frames {
            match subframe.content() {
                id3::Content::Text(text) => {
                    title = Some(text.clone());
                }
                // TODO: Check if anyone uses this method as opposed to `ExtendedLink`.
                id3::Content::Link(url) => {
                    link = Some(Link {
                        url: url::Url::parse(url).map_err(|e| e.to_string())?,
                        title: None,
                    });
                }
                id3::Content::ExtendedLink(extended_link) => {
                    link = Some(Link {
                        url: url::Url::parse(&extended_link.link).map_err(|e| e.to_string())?,
                        title: match extended_link.description.trim() {
                            "" => None,
                            description => Some(description.to_string()),
                        },
                    });
                }
                _ => {}
            }
        }

        chapters.push(Chapter {
            title,
            link,
            start,
            end,
            ..Default::default()
        });
    }

    // Order chapters by start time.
    chapters.sort_by(|a, b| a.start.cmp(&b.start));

    Ok(chapters)
}

/// Writes [chapters](crate::Chapter) to MP3 file's [ID3](https://en.wikipedia.org/wiki/ID3) tag frames.
///
/// If the file already has chapters, they will be replaced.
///
/// # Example:
/// ```rust
/// # use chapters::{Chapter, Link};
/// # use chrono::Duration;
/// # use pretty_assertions::assert_eq;
/// #
/// # fn main() {
/// #     let dst_filepath_str = "tests/data/id3-chapters.jfk-rice-university-speech.frames-added.mp3";
/// #     let dst_filepath = std::path::Path::new(&dst_filepath_str);
/// #
/// #     struct Test {
/// #         src_filepath_str: &'static str,
/// #     }
/// #
/// #     let tests = vec![
/// #         Test {
/// #             src_filepath_str: "tests/data/id3-chapters.jfk-rice-university-speech.mp3",
/// #         },
/// #         Test {
/// #             src_filepath_str: "tests/data/id3-chapters.jfk-rice-university-speech.no-frames.mp3",
/// #         },
/// #     ];
/// #
/// #     for test in tests {
/// #         let src_filepath = std::path::Path::new(&test.src_filepath_str);
/// let chapters = vec![
///     Chapter {
///         start: Duration::seconds(0),
///         title: Some("Introduction".to_string()),
///         link: Some(Link{
///             url: url::Url::parse("https://www.rice.edu").unwrap(),
///             title: None,
///         }),
///         ..Default::default()
///     },
///     Chapter {
///         start: Duration::seconds(42),
///         title: Some("Status quo".to_string()),
///         ..Default::default()
///     },
///     Chapter {
///         start: chrono::Duration::minutes(5) + chrono::Duration::seconds(8),
///         title: Some(String::from("On being first")),
///         link: Some(Link{
///             url: url::Url::parse("https://www.osti.gov/opennet/manhattan-project-history/Events/1945/trinity.htm").unwrap(),
///             title: Some(String::from("The Trinity Test")),
///         }),
///         ..Default::default()
///     },
/// ];
///
/// chapters::to_mp3_file(src_filepath, dst_filepath, &chapters).expect("Failed to write chapters");
/// #
/// #         let chapters_read = chapters::from_mp3_file(dst_filepath).expect("Failed to read chapters");
///           # assert_eq!(chapters, chapters_read);
/// #
/// #         // Cleanup
/// #         std::fs::remove_file(dst_filepath).unwrap();
/// #     }
/// # }
/// ```
pub fn to_mp3_file<P: AsRef<Path>>(
    src_path: P,
    dst_path: P,
    chapters: &[Chapter],
) -> Result<(), String> {
    std::fs::copy(&src_path, &dst_path).map_err(|e| {
        format!(
            "Error copying `{}` to `{}`: {}",
            src_path.as_ref().display(),
            dst_path.as_ref().display(),
            e
        )
    })?;

    let mut tag = match Tag::read_from_path(&src_path) {
        Ok(mut tag) => {
            tag.remove_all_chapters();
            tag
        }
        Err(Error {
            kind: ErrorKind::NoTag,
            ..
        }) => Tag::new(),
        Err(err) => {
            return Err(format!(
                "Error reading ID3 tag from `{}`: {}",
                src_path.as_ref().display(),
                err
            ))
        }
    };

    for (i, chapter) in chapters.iter().enumerate() {
        let mut id3_chapter = id3::frame::Chapter {
            element_id: format!("chp{}", i + 1),
            start_time: chapter.start.num_milliseconds() as u32,
            end_time: if let Some(end) = chapter.end {
                end.num_milliseconds() as u32
            } else {
                chapter.start.num_milliseconds() as u32
            },
            start_offset: 0,
            end_offset: 0,
            frames: Vec::new(),
        };

        if let Some(title) = &chapter.title {
            let frame = id3::frame::Frame::with_content("TIT2", id3::Content::Text(title.clone()));
            id3_chapter.frames.push(frame);
        }

        if let Some(link) = &chapter.link {
            // title or "" if None
            let link_title = link.title.as_ref().map_or("", |t| t.as_str());
            let frame = id3::frame::Frame::with_content(
                "WXXX",
                id3::Content::ExtendedLink(id3::frame::ExtendedLink {
                    link: link.url.to_string(),
                    description: link_title.to_string(),
                }),
            );
            id3_chapter.frames.push(frame);
        }

        tag.add_frame(id3::frame::Frame::with_content(
            "CHAP",
            id3::Content::Chapter(id3_chapter),
        ));
    }

    tag.write_to_path(&dst_path, Version::Id3v24).map_err(|e| {
        format!(
            "Error writing ID3  tag to `{}`: {}",
            dst_path.as_ref().display(),
            e
        )
    })?;

    Ok(())
}
