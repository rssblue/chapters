# Chapters

Manage your podcast (or any media) chapters in multiple formats using Rust.

This crate allows you to extract chapters from

- [x] [JSON chapter files](crate::from_json)
- [x] [MP3 ID3v2 tags](crate::from_mp3_file)
- [x] [Episode show notes](crate::from_description)

## Including in a project

```
cargo add chapters
```

This will include the latest version of the crate in your `Cargo.toml` file.
