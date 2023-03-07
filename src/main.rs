pub mod aggregator;
pub mod spotify_handler;

use aggregator::aggregator::Playlist;
use dotenv::dotenv;
use spotify_handler::spotify_helper::SpotifyHandler;

// todo: there has to be a better way of authenticating spotify

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // load env
    dotenv().ok();
    let mut spotify_handler = SpotifyHandler::new().await;
    let song_titles = match gather_results().await {
        Ok(t) => t,
        Err(..) => panic!("encountered an error")
    };
    
    for x in song_titles {
        spotify_handler.search_for_track(&x).await;
    }

    // create the playlist
    spotify_handler.create_user_playlist().await;

    // once we have titles in handler add them to playlist
    let res = spotify_handler.add_tracks_to_playlist()
        .await;    
    println!("success! playlist created!!");
    dbg!(res);

    Ok(())
}

pub async fn gather_results() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut collect_data = true;
    let mut next_page_token = "".to_string();
    let mut titles: Vec<String> = vec![];
    let mut res;

    let client = reqwest::Client::new();
    
    // as long as we keep getting a next_page, gather results and stuff titles into list
    while collect_data {
        // build the youtube call
        let url = build_youtube_url(
            dotenv::var("PLAYLIST_ID").unwrap(), 
            dotenv::var("YOUTUBE_API_KEY").unwrap(),
            next_page_token);
        res = client.get(&url)
            .send()
            .await?
            .json::<Playlist>()
            .await?;

        for i in res.items {
            titles.push(i.snippet.title);
        }
        // if we have another page token, save bind it, and keep gathering results
        next_page_token = res.next_page_token.unwrap_or("".to_string());

        if next_page_token == "".to_string() {
            collect_data = false;
        }
    }
    return Ok(titles);
}

pub fn build_youtube_url(playlist_id: String, api_key: String, next_page_token: String) -> String {
    let mut query = format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&key={}&maxResults=50",
        playlist_id, api_key
    );
    if next_page_token != "skip" || next_page_token != "" {
        query.push_str(&format!("&pageToken={}", next_page_token));
    };
    return query;
}

