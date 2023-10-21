use cfg_if::cfg_if;
#[cfg(feature = "rssblue")]
use chapters::RemoteEntity;
use chapters::{from_json, Chapter, Image, Link};
use pretty_assertions::assert_eq;

#[test]
fn test_json() {
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
        #[cfg(feature = "rssblue")]
        Test {
            file_contents: include_str!(
                "data/podcast-namespace-chapters.github-example-rssblue-variant.json"
            ),
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
                    remote_entity: Some(RemoteEntity::Item {
                        feed_guid: uuid::Uuid::parse_str("917393e3-1b1e-5cef-ace4-edaa54e1f810")
                            .unwrap(),
                        guid: String::from("44a78abc-dffe-4de2-9230-6d6e723360a5"),
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
        if let Ok(expected) = test.expected.as_ref() {
            let serialized = chapters::to_json(expected).unwrap();
            assert_eq!(serialized.trim(), test.file_contents.trim());
        }

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
            end: Some(chrono::Duration::seconds(10) + chrono::Duration::milliseconds(400)),
            title: Some(String::from("Start")),
            link: Some(Link {
                url: url::Url::parse("https://example.com").unwrap(),
                title: Some(String::from("Example")),
            }),
            image: Some(Image::Url(
                url::Url::parse("https://example.com/image.png").unwrap(),
            )),
            hidden: false,
            #[cfg(feature = "rssblue")]
            remote_entity: Some(RemoteEntity::Item {
                feed_guid: uuid::Uuid::parse_str("917393e3-1b1e-5cef-ace4-edaa54e1f810").unwrap(),
                guid: String::from("44a78abc-dffe-4de2-9230-6d6e723360a5"),
            }),
        },
        Chapter {
            start: chrono::Duration::seconds(10) + chrono::Duration::milliseconds(400),
            end: None,
            title: None,
            link: None,
            image: None,
            hidden: false,
            #[cfg(feature = "rssblue")]
            remote_entity: None,
        },
    ];

    // ensure indentation
    let result = serde_json::to_string_pretty(&chapters).unwrap();

    cfg_if! { if #[cfg( feature = "rssblue" )]{
    let expected = r#"[
  {
    "start": 0,
    "end": 10.4,
    "title": "Start",
    "image": {
      "Url": "https://example.com/image.png"
    },
    "link": {
      "url": "https://example.com/",
      "title": "Example"
    },
    "hidden": false,
    "remote_entity": {
      "item": {
        "feed_guid": "917393e3-1b1e-5cef-ace4-edaa54e1f810",
        "guid": "44a78abc-dffe-4de2-9230-6d6e723360a5"
      }
    }
  },
  {
    "start": 10.4,
    "hidden": false
  }
]"#;
    } else {
    let expected = r#"[
  {
    "start": 0,
    "end": 10.4,
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
    "start": 10.4,
    "hidden": false
  }
]"#;
    }}

    assert_eq!(result, expected);
}
