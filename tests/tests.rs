use chapters::{parse_chapters, Chapter};

#[test]
fn test_parse_chapters() {
    let file_contents = include_str!("data/chapters.json");
    let reader = std::io::BufReader::new(file_contents.as_bytes());
    let chapters = chapters::parse_chapters(reader);

    let expected = Ok(vec![
        Chapter {
            start: chrono::Duration::seconds(0),
            title: Some(String::from("Intro")),
            ..Default::default()
        },
        Chapter {
            start: chrono::Duration::seconds(168),
            title: Some(String::from("Hearing Aids")),
            image_url: Some(
                url::Url::parse("https://example.com/images/hearing_aids.jpg").unwrap(),
            ),
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
            image_url: Some(
                url::Url::parse("https://example.com/images/namepsace_example.jpg").unwrap(),
            ),
            url: Some(
                url::Url::parse("https://github.com/Podcastindex-org/podcast-namespace").unwrap(),
            ),
            ..Default::default()
        },
        Chapter {
            start: chrono::Duration::seconds(3990),
            title: Some(String::from("Just Break Up")),
            image_url: Some(
                url::Url::parse("https://example.com/images/justbreakuppod.png").unwrap(),
            ),
            ..Default::default()
        },
        Chapter {
            start: chrono::Duration::seconds(4600),
            title: Some(String::from("Donations")),
            url: Some(url::Url::parse("https://example.com/paypal_link").unwrap()),
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
    ]);

    assert_eq!(chapters, expected);
}
