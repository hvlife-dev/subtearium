# Subtearium

## An arr* stack element, responsible for downloading, handling, and editing music subtitles

Dev stack is based on Rust + Leptos + Axum. 
Lyrics source is indispensable lrclib.

Main features:
 * Support for sync, and plain lyrics (configurable)
 * Clear reporting of each library entry state
 * Easy lyrics locking, to prevent overwrite of unwanted files
 * Quick sync correction, by shifting synced lyrics timestaps +/- seconds
 * New library entries detection
 * Automatic searching for missing or incomplete lyrics on time interval

This project is in beta stage, so breaking changes and unstability are to be expected.
All contributions and suggestions are welcome, as well as bug reports.
Before using, please backup your library, this service shouldn't affect files other than .lrc, but I don't wan't to be responsible for you losing your hard collected data.

Docker is a preffered way of running Subtearium, dockerhub image will be available as soon as I'm done with main project structure...
Upon starting you can access web UI at `0.0.0.0:2137`.

You can run the project in dev mode with `cargo leptos watch`, more insight in the Leptos Axum tutorial.

<img src="./public/neon.svg">
