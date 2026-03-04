# Subtearium

**Music lyrics manager that can run as a part of your \*arr stack server.**

Built with **Rust + Leptos + Axum**. It utilizes the indispensable [LRCLIB](https://lrclib.net/) as its primary lyrics source.

### Main Features
* **Lightweight:** Minimal RAM and CPU footprint thanks to the Rust backend.
* **Flexible Lyrics Support:** Configurable support for downloading both synced and plain text lyrics.
* **Library Reporting:** Clear, visual reporting of the state of every entry in your library.
* **Lyrics Locking:** Easily lock specific tracks to prevent the engine from overwriting manually added or custom files.
* **Timestamp Shifting:** Quick sync correction allows you to shift synced lyrics timestamps forward or backward directly in the UI.
* **Smart Searching:** Search instantly for new media entries, while using interval for incomplete ones.
* **Responsive UI:** Fully functional mobile UI.

> [!WARNING]
> This project is currently in beta. Breaking changes and instability are to be expected.
> **Please back up your library before using.** While this service is strictly designed to only create and modify `.lrc` files, I am not responsible for any accidental loss of your heavily curated media data. 

---

## Setup

Docker is the preferred way of running Subtearium. Below is an example `docker-compose.yml` snippet:

```yaml
services:
  subtearium:
    container_name: subtearium
    image: hvlife/subtearium:latest
    environment:
      - PUID=1000
      - PGID=1000
    ports:
      - 2137:2137/tcp
    volumes:
      - /docker/appdata/subtearium:/app/data:rw  # App config
      - /mnt/data/media/music:/music:rw          # Your music library
    restart: unless-stopped
```

Upon starting the container, you can access the web UI at `http://<your-server-ip>:2137`.

---

## Screenshots

![Home Dashboard](./public/readme_home.png)
![Status Page](./public/readme_status.png)
![Edit Lyrics](./public/readme_edit.png)
![Settings](./public/readme_settings.png)

### RAM Usage Comparison (Over 10k songs)
![Dozzle RAM Usage](./public/readme_dozzle.png)

---

## Development

You can run the project locally in development mode using `cargo-leptos`:

```bash
cargo leptos watch
```

*Note: Ensure your `wasm-bindgen` version matches the one used by your Leptos setup. For more insights into the architecture, check out the official [Leptos Axum page](https://leptos.dev).*

**Contributing:** All contributions, suggestions, and bug reports are highly welcome.
