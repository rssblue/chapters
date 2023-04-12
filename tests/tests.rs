use chapters::{parse_chapters, Chapter, Image, Link};
use pretty_assertions::assert_eq;

#[test]
fn test_parse_chapters() {
    struct Test {
        file_contents: &'static str,
        expected: Result<Vec<Chapter>, String>,
    }

    let tests = vec![
        Test {
            file_contents: include_str!("data/podcast-namespace-chapters.github-example.json"),
            expected: Ok(vec![
                Chapter {
                    start: chrono::Duration::seconds(0),
                    title: Some(String::from("Intro")),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(168),
                    title: Some(String::from("Hearing Aids")),
                    image: Some(Image::Url(
                        url::Url::parse("https://example.com/images/hearing_aids.jpg").unwrap(),
                    )),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(260),
                    title: Some(String::from("Progress Report")),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(410),
                    title: Some(String::from("Namespace")),
                    image: Some(Image::Url(
                        url::Url::parse("https://example.com/images/namepsace_example.jpg")
                            .unwrap(),
                    )),
                    link: Some(Link {
                        url: url::Url::parse(
                            "https://github.com/Podcastindex-org/podcast-namespace",
                        )
                        .unwrap(),
                        title: None,
                    }),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(3990),
                    title: Some(String::from("Just Break Up")),
                    image: Some(Image::Url(
                        url::Url::parse("https://example.com/images/justbreakuppod.png").unwrap(),
                    )),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(4600),
                    title: Some(String::from("Donations")),
                    link: Some(Link {
                        url: url::Url::parse("https://example.com/paypal_link").unwrap(),
                        title: None,
                    }),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(5510),
                    title: Some(String::from("The Big Players")),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(5854),
                    title: Some(String::from("Spread the Word")),
                    ..Default::default()
                },
                Chapter {
                    start: chrono::Duration::seconds(6089),
                    title: Some(String::from("Outro")),
                    ..Default::default()
                },
            ]),
        },
        Test {
            file_contents: include_str!("data/podcast-namespace-chapters.empty.json"),
            expected: Ok(vec![]),
        },
    ];

    for test in tests {
        let reader = std::io::BufReader::new(test.file_contents.as_bytes());
        let result = parse_chapters(reader);

        assert_eq!(result, test.expected);
    }
}

#[test]
fn test_chapters_from_description() {
    struct Test {
        description: &'static str,
        expected: Result<Vec<Chapter>, String>,
    }

    let tests = vec![Test {
        description: include_str!("data/description-chapters.txt"),
        expected: Ok(vec![
            Chapter {
                start: chrono::Duration::seconds(0),
                title: Some(String::from("Intro")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(4) + chrono::Duration::seconds(45),
                title: Some(String::from("Plot summary")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(10) + chrono::Duration::seconds(11),
                title: Some(String::from("Sergio Leone")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(16) + chrono::Duration::seconds(58),
                title: Some(String::from("Ennio Morricone")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(22) + chrono::Duration::seconds(30),
                title: Some(String::from("Charles Bronson")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(27) + chrono::Duration::seconds(22),
                title: Some(String::from("Henry Fonda")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(32) + chrono::Duration::seconds(21),
                title: Some(String::from("Conclusion")),
                ..Default::default()
            },
        ]),
    }];

    for test in tests {
        let result = chapters::chapters_from_description(test.description);

        assert_eq!(result, test.expected);
    }
}

#[test]
fn test_chapters_from_mp3_file() {
    struct Test {
        file_path: &'static str,
        expected: Result<Vec<Chapter>, String>,
    }

    let tests = vec![
        Test {
        file_path: "tests/data/id3-chapters.jfk-rice-university-speech.mp3",
        expected: Ok(vec![
            Chapter {
                start: chrono::Duration::seconds(0),
                title: Some(String::from("Introduction")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::seconds(9),
                title: Some(String::from("Thanks")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::seconds(42),
                title: Some(String::from("Status quo")),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(5) + chrono::Duration::seconds(8),
                title: Some(String::from("On being first")),
                link: Some(Link{
                    url: url::Url::parse("https://www.osti.gov/opennet/manhattan-project-history/Events/1945/trinity.htm").unwrap(),
                    title: Some(String::from("The Trinity Test")),
                }),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(8) + chrono::Duration::seconds(8),
                title: Some(String::from("Why we're going to the Moon")),
                link: Some(Link{
                    url: url::Url::parse("https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html").unwrap(),
                    title: None,
                }),
                ..Default::default()
            },
            Chapter {
                start: chrono::Duration::minutes(16) + chrono::Duration::seconds(24),
                title: Some(String::from("Conclusion")),
                ..Default::default()
            },
        ]),
    },
        Test {
            file_path: "tests/data/id3-chapters.jfk-rice-university-speech.no-frames.mp3",
            expected: Ok(vec![]),
        },
    ];

    for test in tests {
        let path = std::path::Path::new(test.file_path);
        let result = chapters::chapters_from_mp3_file(path);

        assert_eq!(result, test.expected);
    }
}
