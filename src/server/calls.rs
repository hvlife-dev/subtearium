use reqwest::Client;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LrcTrack {
    pub id: u64,
    pub track_name: String,
    pub artist_name: String,
    pub album_name: String,
    pub duration: f64, 
    pub instrumental: bool,
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct LrcError {
    pub code: u16,
    pub name: String,
    pub message: String,
}

pub async fn get_lyric(title: &str, artist: &str, album: &str, duration: &str) -> Result<LrcTrack, LrcError> {
    let client = Client::new();
    let url = "https://lrclib.net/api/get";

    let params = [
        ("artist_name", &artist),
        ("track_name", &title),
        ("album_name", &album),
        ("duration", &duration),
    ];

    leptos::logging::log!("Sending request...");

    let response_result = client
        .get(url)
        .query(&params)
        .send()
        .await;

    match response_result {
        Ok(response) => {
            if response.status().is_success() {
                let body_text = response.text().await.unwrap_or_default();
                // leptos::logging::log!("Raw JSON: {}", body_text);

                match serde_json::from_str::<LrcTrack>(&body_text) {
                    Ok(track) => {
                        leptos::logging::log!("--- Success ---");
                        leptos::logging::log!("Track: {}", track.track_name);
                        if track.synced_lyrics.is_some() {leptos::logging::log!("Synced");}
                        if track.plain_lyrics.is_some() {leptos::logging::log!("Plain");}
                        Ok(track)
                    }
                    Err(e) => {
                        leptos::logging::log!("Parsing Failed: {}", e);
                        Err(LrcError{
                            code: 2137, 
                            name: "serde".to_string(), 
                            message: "Json parsing error".to_string()})
                    }
                }
            } else {
                match response.json::<LrcError>().await {
                    Ok(err) => {
                        leptos::logging::log!("API Error: {}", err.message);
                        Err(err)
                    },
                    Err(_) => {
                        leptos::logging::log!("Unknown API Error");
                        Err(LrcError{
                            code: 2138, 
                            name: "reqwest".to_string(), 
                            message: "Unknown API error".to_string()})
                    },
                }
            }
        }
        Err(e) => {
            leptos::logging::log!("Network Error: {}", e);
            Err(LrcError{
                code: 2139, 
                name: "Network".to_string(), 
                message: "Connection error".to_string()})

        },
    }
}
