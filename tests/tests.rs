use chapters::{from_json, Chapter, Image, Link};
use pretty_assertions::assert_eq;

#[test]
fn test_from_json() {
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
        let result = from_json(reader);

        assert_eq!(result, test.expected);
    }
}

#[test]
fn test_from_description() {
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
        let result = chapters::from_description(test.description);

        assert_eq!(result, test.expected);
    }
}

#[test]
fn test_to_json() {
    let chapters = vec![
        Chapter {
            start: chrono::Duration::seconds(0),
            end: Some(chrono::Duration::seconds(10)),
            title: Some(String::from("Start")),
            link: Some(Link {
                url: url::Url::parse("https://example.com").unwrap(),
                title: Some(String::from("Example")),
            }),
            image: Some(Image::Url(
                url::Url::parse("https://example.com/image.png").unwrap(),
            )),
            hidden: false,
        },
        Chapter {
            start: chrono::Duration::seconds(10),
            end: None,
            title: None,
            link: None,
            image: None,
            hidden: false,
        },
    ];

    // ensure indentation
    let result = serde_json::to_string_pretty(&chapters).unwrap();

    assert_eq!(
        result,
        r#"[
  {
    "start": 0.0,
    "end": 10.0,
    "title": "Start",
    "image": {
      "Url": "https://example.com/image.png"
    },
    "link": {
      "url": "https://example.com/",
      "title": "Example"
    },
    "hidden": false
  },
  {
    "start": 10.0,
    "hidden": false
  }
]"#
    );
}
