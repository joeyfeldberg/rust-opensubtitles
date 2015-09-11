[![Build Status](https://travis-ci.org/joeyfeldberg/rust-opensubtitles.svg?branch=master)](https://travis-ci.org/joeyfeldberg/rust-opensubtitles)

Toy wrapper for opensubtitles API, this is not for any serious use of course.
Just a fun little toy that can search a matching subtitle for a given movie file path.
It includes an example implementation that can search recursively a given path for movie files and download their matching subtitle.

Build lib:
```
cargo build
```

Build sample application:
```
cargo build --bin opensubtitles-downloader
```

Example of using the opensubtitles-downloader:
```
opensubtitles-downloader /some/path eng
```

This will recursively traverse all sub directories of /some/path and will try to download subtitles for any video file it finds.
