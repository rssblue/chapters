#![doc = include_str!("../README.md")]

mod serialization;

use chrono::Duration;
use id3::Tag;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize)]
pub struct Link {
    #[serde(serialize_with = "serialization::url_to_string")]
    pub url: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Image {
    Url(url::Url),
    // TODO: some ways of encoding chapters (e.g., ID3 tags in MP3 files) allow to embed images directly in the file.
    // Data(Vec<u8>),
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
        }
    }
}

impl From<PodcastNamespaceChapter> for Chapter {
    fn from(podcast_namespace_chapter: PodcastNamespaceChapter) -> Self {
        Self {
            start: podcast_namespace_chapter.start_time,
            end: podcast_namespace_chapter.end_time,
            title: podcast_namespace_chapter.title,
            image: match podcast_namespace_chapter.img {
                Some(url) => Some(Image::Url(url)),
                None => None,
            },
            link: match podcast_namespace_chapter.url {
                Some(url) => Some(Link { url, title: None }),
                None => None,
            },
            hidden: !podcast_namespace_chapter.toc.unwrap_or(true),
        }
    }
}

/// Chapters of the [Podcast namespace](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
#[derive(Debug, PartialEq, Deserialize)]
struct PodcastNamespaceChapters {
    version: String,
    chapters: Vec<PodcastNamespaceChapter>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PodcastNamespaceChapter {
    /// The starting time of the chapter.
    #[serde(deserialize_with = "serialization::float_to_duration")]
    start_time: Duration,
    /// The end time of the chapter.
    #[serde(default, deserialize_with = "serialization::float_to_duration_option")]
    end_time: Option<Duration>,
    /// The title of this chapter.
    #[serde(default)]
    title: Option<String>,
    /// The url of an image to use as chapter art.
    #[serde(default, deserialize_with = "serialization::string_to_url")]
    img: Option<url::Url>,
    /// The url of a web page or supporting document that's related to the topic of this chapter.
    #[serde(default, deserialize_with = "serialization::string_to_url")]
    url: Option<url::Url>,
    /// If this property is present and set to false, this chapter should not display visibly to the user in either the table of contents or as a jump-to point in the user interface.
    #[serde(default)]
    toc: Option<bool>,
    // TODO: This object defines an optional location that is tied to this chapter.
    // pub location: Option<()>,
}

pub fn from_json<R: std::io::Read>(reader: R) -> Result<Vec<Chapter>, String> {
    let podcast_namespace_chapters: PodcastNamespaceChapters =
        serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(podcast_namespace_chapters
        .chapters
        .into_iter()
        .map(|c| c.into())
        .collect())
}

#[derive(Debug, Clone)]
enum TimestampType {
    /// MM:SS format, e.g., "12:34"
    MMSS,
    /// HH:MM:SS format, e.g., "01:23:45"
    HHMMSS,
    /// MM:SS format within parentheses, e.g., "(12:34)"
    MMSSParentheses,
    /// HH:MM:SS format within parentheses, e.g., "(01:23:45)"
    HHMMSSParentheses,
}

impl TimestampType {
    fn regex_pattern(&self) -> &str {
        match self {
            Self::MMSS => r"^(?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)",
            Self::HHMMSS => r"^(?P<hours>\d{2}):(?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)",
            Self::MMSSParentheses => r"^\((?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)\)",
            Self::HHMMSSParentheses => {
                r"^\((?P<hours>\d{2}):(?P<minutes>[0-5]\d):(?P<seconds>[0-5]\d)\)"
            }
        }
    }

    fn line_regex_pattern(&self) -> String {
        // Combines the timestamp regex pattern with space (or a punctuation mark) and a pattern for text following the timestamp.
        format!("{}[.!?\\- ](?P<text>.+)$", self.regex_pattern())
    }

    fn from_line(line: &str) -> Option<Self> {
        if let Some(first_char) = line.chars().next() {
            if first_char == '(' || first_char.is_numeric() {
                return [
                    Self::MMSS,
                    Self::HHMMSS,
                    Self::MMSSParentheses,
                    Self::HHMMSSParentheses,
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
                title: Some(text.to_string()),
                image: None,
                link: None,
                hidden: false,
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

/// Reads chapter frames in ID3 tags.
pub fn from_mp3_file<P: AsRef<Path>>(path: P) -> Result<Vec<Chapter>, String> {
    let tag = Tag::read_from_path(path).map_err(|e| format!("Error reading ID3 tag: {}", e))?;
    let mut chapters = Vec::new();

    for frame in tag.frames() {
        let id3_chapter = match frame.content() {
            id3::Content::Chapter(chapter) => chapter,
            _ => {
                continue;
            }
        };

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
