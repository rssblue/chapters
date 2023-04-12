#![doc = include_str!("../README.md")]

use chrono::Duration;
use serde::Deserialize;

/// Chapters follow mostly the [Podcast namespace specification](https://github.com/Podcastindex-org/podcast-namespace/blob/main/chapters/jsonChapters.md).
#[derive(Debug, PartialEq)]
pub struct Chapter {
    /// The starting time of the chapter.
    pub start: Duration,
    /// The end time of the chapter.
    pub end: Option<Duration>,
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
            start: Duration::zero(),
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
    pub start_time: Duration,
    /// The end time of the chapter.
    #[serde(default, deserialize_with = "float_to_duration_option")]
    pub end_time: Option<Duration>,
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

fn float_to_duration_option<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f = match Option::<f64>::deserialize(deserializer) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };
    Ok(f.map(|f| Duration::seconds(f as i64)))
}

fn float_to_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let f = f64::deserialize(deserializer)?;
    Ok(Duration::seconds(f as i64))
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

pub fn chapters_from_description(description: &str) -> Result<Vec<Chapter>, String> {
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
                image_url: None,
                url: None,
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
