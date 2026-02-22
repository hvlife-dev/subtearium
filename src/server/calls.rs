use reqwest::Client;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

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

pub async fn get_lyric(client: &Client, title: &str, artist: &str, album: &str, duration: &str) -> Result<LrcTrack, LrcError> {
    let url = "https://lrclib.net/api/get";

    let params = [
        ("artist_name", artist),
        ("track_name", title),
        ("album_name", album),
        ("duration", duration),
    ];

    let max_retries = 3;
    let mut attempt = 0;

    loop {
        attempt += 1;

        let response_result = client
            .get(url)
            .query(&params)
            .send()
            .await;

        match response_result {
            Ok(response) => {
                if response.status().is_success() {
                    let body_text = response.text().await.unwrap_or_default();
                    return match serde_json::from_str::<LrcTrack>(&body_text) {
                        Ok(track) => Ok(track),
                        Err(e) => Err(LrcError {
                            code: 2137, 
                            name: "serde".to_string(), 
                            message: e.to_string()
                        }),
                    };
                } else if response.status().as_u16() == 429 {
                    if attempt >= max_retries {
                        return Err(LrcError {
                            code: 429,
                            name: "RateLimit".to_string(),
                            message: "Max retries exceeded due to rate limiting.".to_string(),
                        });
                    }
                    
                    let backoff = attempt * 3; 
                    sleep(Duration::from_secs(backoff as u64)).await;
                    continue; 
                    
                } else {
                    let code = response.status().as_u16();
                    let message = response.text().await.unwrap_or_default();
                    return match serde_json::from_str::<LrcError>(&message) {
                        Ok(err) => Err(err),
                        Err(_) => Err(LrcError {
                            code, 
                            name: "ApiError".to_string(), 
                            message
                        }),
                    };
                }
            }
            Err(e) => {
                if attempt >= max_retries {
                    return Err(LrcError {
                        code: 2139, 
                        name: "Network".to_string(), 
                        message: e.to_string()
                    });
                }
                
                sleep(Duration::from_secs(2)).await;
                continue;
            }
        }
    }
}
